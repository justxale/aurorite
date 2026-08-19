<script lang="ts" setup>
import type { HTMLAttributes } from "vue"
import { ErrorMessage } from "vee-validate"
import { toValue } from "vue"
import { cn } from "@/lib/utils"
import { useFormField } from "./useFormField"

const props = defineProps<{
  class?: HTMLAttributes["class"]
}>()

const { name, formMessageId } = useFormField()
const { t } = useI18n()
</script>

<template>
  <ErrorMessage
    :id="formMessageId"
    v-slot="{ message }"
    data-slot="form-message"
    as="p"
    :name="toValue(name)"
    :class="cn('text-destructive text-sm mx-6 ps-2 md:ps-4 text-[1rem]', props.class)"
  >
      <template v-if="message">
          {{ t(message) }}
      </template>
  </ErrorMessage>
</template>
