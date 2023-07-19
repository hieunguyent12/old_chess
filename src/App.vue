<script setup lang="ts">
import HelloWorld from "./components/HelloWorld.vue";
import TheWelcome from "./components/TheWelcome.vue";
import YeetStuff from "./components/YeetStuff.vue";
import { computed, reactive, ref } from "vue";

import { New as LMAO } from "chess-wasm";

let dumb = 0;

let count = ref({
  count: 0,
  count2: 0,
});

let classObj = ref({
  active: true,
  "text-danger": true,
});

let count2 = reactive({ count: 0 });

function updateCount() {
  // init.greet();
  // greet();

  let a = LMAO.new();

  console.log(a.get_test());
}

const isOdd = computed(() => {
  return count.value.count2 % 2 !== 0;
});
</script>

<template>
  <div>
    <p>Chess!!</p>

    <p @click="updateCount" :class="classObj">Counter: {{ count.count2 }}</p>
    <p>Is it odd? {{ isOdd }}</p>

    <YeetStuff :title="dumb" />
  </div>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

p {
  font-size: 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
