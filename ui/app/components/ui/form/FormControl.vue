<script lang="ts" setup>
import { Slot } from "reka-ui"
import { useFormField } from "./useFormField"

const { error, formItemId, formDescriptionId, formMessageId } = useFormField()
</script>

<template>
  <Slot
    :id="formItemId"
    data-slot="form-control"
    :aria-describedby="!error ? `${formDescriptionId}` : `${formDescriptionId} ${formMessageId}`"
    :aria-invalid="!!error"
    class="mx-6 ps-2 md:ps-4 text-[1rem]"
  >
    <slot />
  </Slot>
</template>
