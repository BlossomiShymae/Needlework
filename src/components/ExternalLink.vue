<template>
  <a
    :class="`${cssClass} text-decoration-none text-dark`"
    :href="href"
    target="_blank"
    ><slot></slot
  ></a>
</template>

<script lang="ts" setup>
const props = defineProps<{
  href: string;
  button?: boolean;
}>();

let cssClass = "";
if (props.button) {
  cssClass = "btn btn-secondary hover-dim";
} else {
  cssClass = "hover-underline";
}
</script>
