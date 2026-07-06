<script setup lang="ts">

const regBlocks = [
  // placeholder - text for 'placeholder';
  // transfer - sending value;
  {placeholder: "Login", transfer: "login"},
  {placeholder: "Password", transfer: "password"}
]

</script>

<template>
    <div class="w-full">
        <div class="relative flex z-0 bg-grey h-screen text-black rounded-b-xl border-solid">
            <div class="z-2 w-full h-full absolute m-0 text-black rounded-sm border-solid opacity-50 bg-[linear-gradient(360deg,#66a8ff_75%,transparent)] [clip-path:polygon(0_0,90%_0%,90%_70%,0_40%)]"/>
            <div class="z-1 w-full h-full absolute m-0 text-black rounded-sm border-solid [clip-path:polygon(0_0,100%_0,100%_15%,0_40%)] bg-[linear-gradient(180deg,#6dffaf_95%,transparent)] opacity-70"/>
            <div class="z-3 w-full h-full absolute m-0 text-black rounded-sm border-solid [clip-path:polygon(0_40%,90%_17.5%,90%_70%,0_100%)] bg-[radial-gradient(ellipse_at_75%_50%,#c273ff_0,transparent_75%)] opacity-40"/>
            <div class="relative z-3 w-full h-full justify-items-center">
                <div class="absolute z-3 w-[80%] md:w-[50%] lg:w-[40%] h-[35%] justify-items-center top-[30%]">
                    <div class="w-full h-full bg-white mix-blend-normal rounded-[10px]">
                        <div v-for="block in regBlocks" :key="block.placeholder" >
                            <div class="pt-4">
                                <input :id="block.transfer" :placeholder="block.placeholder" class="w-[calc(100%-16px)] my-1 md:my-0 mx-2 p-1 md:p-2 ps-1 md:ps-4" >
                            </div>
                        </div>
                    </div>
                    <button class="transition duration-300 ease-in-out hover:scale-110 cursor-pointer absolute w-[50%] h-[15%] bg-greyBlue rounded-[20px] mix-blend-normal top-[70%] left-[25%]">
                      Register
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

