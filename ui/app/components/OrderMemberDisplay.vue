<script setup lang="ts">

const weaponList = ref([])


</script>

<template>
    <div>
        <div class="w-full h-[25%]">
            <h1 v-for="(weapon, id) in weaponList" :key="id"  class="my-2 bg-foreground rounded-sm"> {{weapon}} </h1>
        </div>
    </div>
</template>
