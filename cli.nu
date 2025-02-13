def "log success" [msg: string] {
  print ($" (ansi green)>(ansi reset) " + $msg)
}

def "log error" [msg: string] {
  print -e ($" (ansi light_red_bold)> " + $msg + (ansi reset))
}

def "show-menu" [--display (-d): cell-path = "render", --loop (-l) --path (-p): list<string>, callback] : table -> table {
  let input = $in

  let callback = if ($callback | describe | str starts-with "record") {
    {|it| 
      if $it == null { return false }

      $it 
      | get -i code
      | if $in == null {
        false
      } else {
        $callback 
        | get -i ($it | get -i code) 
        | if $in != null {
          do $in $it 
        } else {
          log error ("Unknown action: " + ($it | get -i code | to json));
          false
        }
      }
    }
  } else {
    {|it| 
      if $it == null {
        false
      } else {
        do $callback $it
      }
    }
  }

  let title = $"(ansi magenta_underline)/" + ($path | str join /) + $"(ansi reset)\nChoose action"

  if $loop {
    loop {
      if (do $callback ($input | input list -d render $title)) == false {
        break
      }
    }
  } else {
    if (do $callback ($input | input list -d render $title)) == false {
      break
    }
  }
}

def "create-menu" [data: table, --display (-d): cell-path = "render", callback] : closure {
  {|| $data | show-menu -d $display $callback}
}

let temp_file = [$.TEMP $.TMP $.TMPDIR $.TEMPDIR] | each {|it| $env | get -i $it } | compact | get -i 0 | default (pwd) | path join rustlanges.forms.cli.toml
def "load config" [] {
  try { 
    open $temp_file -r | from toml 
  } catch {
    touch $temp_file;
    {}
  }
}

def "save config" [] : table -> nothing {
  $in | save -f $temp_file
}

def close [] {
  print $"Bye! (ansi light_magenta_bold):kisses:(ansi reset)"
  exit 0
}

def "select environment" [] {
  mut config = load config

  let envs = [
    [name url];
    [Local http://localhost:8787]
    [Remote https://forms-api.rustlanges.workers.dev]
  ] 

  let last_env = $config | get -i last_env

  let envs = if $last_env != null {
    $envs | prepend {name: $"($last_env.name)", url: $last_env.url, last_env: true}
  } else {
    $envs
  }

  loop {
    let e = $envs
    | each {|it| 
      $it
      | insert ping ($in | try { http get -e ($in.url); true } catch { false })
      | insert ping-icon (if $in.ping { char elevated } else { char moon })
      | insert name-color (if $in.ping { ansi green } else { ansi red })
      | insert sub-name ($in | get -i last_env | if $in != null { $" (ansi grey70)Last Selection(ansi reset)" })
      | insert render $"($in.ping-icon) ($in.name-color)($in.name)(ansi reset)($in.sub-name) (ansi grey42)\(($in.url)\)(ansi reset)"
    } 
    | input list -d render "Choose environment";

    match $e {
      { ping: true } => {
        log success $"Using (ansi light_green_bold)($e.name)(ansi reset) environment"
        
        $config.last_env = { name: $e.name, url: $e.url }
        $config | save config
        
        return $e
      }
      { ping: false } => {
        log error "Selected environment is not available"
      }
      _ => {
        close
      }
    }
  }
}

def "github login" [app_env] : string {
  let app_url = $app_env.url

  let github_url = $app_url + "/api/login/github"

  let redirect_url = http get -f -R m $github_url 
    | get headers.response 
    | transpose -rd 
    | get location

  loop {
    print "Hit enter to open browser or backspace to show url...\n"
    match (input listen) {
      { code: "enter" } => { start $redirect_url; break }
      { code: "backspace" } => {
        print ($redirect_url);
        input -s "Hit enter to continue...\n"
        break
      }
      _ => { continue }
    }
  }

  input -s "Paste your token here: \n"
}

def print-login [] {
  let config = load config

  let external_token = $config | get -i external_token

  if $external_token != null { 
    log success "Logged in"
    true
  } else {
    log error "Not logged in"
    false
  }
}

print $"(ansi white_bold)Welcome :D(ansi reset)"

let app_env = select environment

let create_form = {|admin_session|
  clear
  print "Creating Form :cool:"

  let name = input $" (ansi green_bold) Title > (ansi light_magenta_bold)"

  let edition = input $" (ansi green_bold) Edition > (ansi light_magenta_bold)"

  let require_login = [yes no] | input list $" (ansi green_bold) Require login(ansi light_magenta_bold)"
  print $" (ansi green_bold) Require login > (ansi light_magenta_bold)($require_login)"

  let multiple_times = [yes no] | input list $" (ansi green_bold) Multiple times(ansi light_magenta_bold)"
  print $" (ansi green_bold) Multiple times > (ansi light_magenta_bold)($multiple_times)"

  let body = {
    title: $name,
    edition: $edition,
    require_login: ($require_login == yes),
    multiple_times: ($multiple_times == yes),
  } | to json
  let new_form = http post -e -H [authorization ("Bearer " + $admin_session) content-type application/json] ($app_env.url + "/api/form") $body

  # print $new_form
  print ($new_form | get -i errors | default $new_form)
}

let menu_admin = {||
  let external_token = load config 
    | get -i external_token 
    | if $in == null { return true } else { $in }

  let admin_session = http get -H [authorization ("Bearer " + $external_token) user-agent Forms-Cli] ($app_env.url + "/api/session") | get data

  [
    [code render];
    [create "Create Form"]
    [edit "Edit Forms"]
    [Delete "Delete Forms"]
    [exit Back]
  ] 
  | show-menu -p [admin] -l {
    create: {|| do $create_form $admin_session}
    exit: {|| false}
  }
}

print-login

print ""

[
  [code render];
  [admin $"Enter as (ansi light_purple)admin(ansi reset) 😈 "]
  [gh "Github Login"]
  [exit Exit]
] | show-menu -l {
  "admin": $menu_admin
  "gh": {||
    let token = github login $app_env
    
    let config = load config 

    try {
      $config | insert external_token $token 
    } catch {
      $config | update external_token $token 
    }
      | to toml
      | save config

    print-login
  }
  "exit": {|| close }
}

close
