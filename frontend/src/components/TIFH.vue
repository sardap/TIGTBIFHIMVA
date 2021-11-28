<template>
  <div id="tifh">
    <p>
      Request GET: <a :href="request_url">"{{ request_url }}"</a>
    </p>
    <div v-if="message">
      <p>
        {{ message }}
      </p>
      <p v-if="time">
        {{ time }}
      </p>
      <br />
      <p>Powered by</p>
      <p>TIGTBIFHIMVA</p>
    </div>
    <div v-else>
      <p>Loading</p>
    </div>
  </div>
</template>

<style scoped>
p {
  margin-left: 20px;
  margin-right: 20px;
}

#tifh {
  -moz-box-shadow: 0 0 3px #ccc;
  -webkit-box-shadow: 0 0 3px #ccc;
  box-shadow: 0 0 3px #ccc;
  padding: 5px;
  width: 500px;
  margin: 0 auto;
  text-align: center;
  background-color: #fef9eb;
  max-width: 90%;
  word-break: break-all;
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
          this.time = data.result;
          this.message = `Time in Melbourne Victoria Australia in four hours:`;
        } else {
          this.message = "oopise whoopise i made a fucky wucky";
          this.time = null;
        }
      } else {
        this.message = await res.text();
        this.time = null;
      }
    },
  },
  data() {
    return {
      message: null,
      time: true,
      request_url: `https://tigtbifhimva.sarda.dev/api/v1/time_in_4_hours?return_type=${this.return_type}`,
    };
  },
})
export default class TIFH extends Vue {}
</script>

<style scoped></style>
