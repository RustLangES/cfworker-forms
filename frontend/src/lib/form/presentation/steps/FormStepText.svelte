<script lang="ts">
	import type { FormStepText } from "../../models/Step.d";

  export let step: FormStepText;
</script>

<h1> {step.data.title} </h1>

<p> {step.data.description} </p>
