<template>
  <div id="tifh">
    <p>
      Request GET: <a :href="request_url">"{{ request_url }}"</a>
    </p>
    <div v-if="message">
      <p>
        {{ message }}
      </p>
      <p>Powered by TIGTBIFHIMVA</p>
    </div>
    <div v-else>
      <p>Loading</p>
    </div>
  </div>
</template>

<style scoped>
#tifh {
  -moz-box-shadow: 0 0 3px #ccc;
  -webkit-box-shadow: 0 0 3px #ccc;
  box-shadow: 0 0 3px #ccc;
  padding: 5px;
  width: 500px;
  margin: 0 auto;
  text-align: center;
  background-color: #fef9eb;
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    return_type: String,
  },
  created() {
    this.fetchData();
    this.timer = setInterval(this.fetchData, 1000);
  },
  methods: {
    async fetchData() {
      const res = await fetch(this.request_url);
      if (this.return_type === "json") {
        const data = await res.json();
        if (data.result !== null) {
          this.message = `Time in Melbourne Victoria Australia in four hours: ${data.result}`;
        } else {
          this.message = "oopise whoopise i made a fucky wucky";
        }
      } else {
        this.message = await res.text();
      }
    },
  },
  data() {
    return {
      message: null,
      request_url: `https://tigtbifhimva.sarda.dev/api/v1/time_in_4_hours?return_type=${this.return_type}`,
    };
  },
})
export default class TIFH extends Vue {}
</script>

<style scoped></style>
