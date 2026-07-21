<script setup lang="ts">

</script>

<template>
  <div class="w-screen h-screen">
    <div class="absolute w-full h-full m-0 text-black rounded-sm border-solid [clip-path:polygon(0_0,100%_0,100%_15%,0_40%)] bg-[linear-gradient(180deg,#6dffaf_95%,transparent)] opacity-70"/>
    <div class="absolute w-full h-full m-0 text-black rounded-sm border-solid opacity-50 bg-[linear-gradient(360deg,#66a8ff_75%,transparent)] [clip-path:polygon(0_0,90%_0%,90%_70%,0_40%)]"/>
    <div class="absolute w-full h-full m-0 text-black rounded-sm border-solid [clip-path:polygon(0_40%,90%_17.5%,90%_70%,0_100%)] bg-[radial-gradient(ellipse_at_75%_50%,#c273ff_0,transparent_75%)] opacity-40"/>
    <div class="flex bg-grey h-full text-black rounded-b-xl border-solid">
      <slot/>
    </div>
  </div>
</template>
