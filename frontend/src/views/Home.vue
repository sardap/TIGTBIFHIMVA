<template>
  <div class="home">
    <div class="section-top">
      <h1 class="title">
        The days of not knowing what the time it's going to be in four hours in
        Melbourne Victoria Australia are over!
      </h1>
    </div>
    <hr />
    <div class="section" id="intro">
      <row container :columns="2">
        <column>
          <img
            width="300"
            src="https://media.istockphoto.com/vectors/happy-computer-vector-id483855921"
          />
        </column>
        <column class="cn">
          <div>
            <h2>A new age is upon us</h2>
            <p class="sub-heading">The old world is dead.</p>
            <p>
              The only API which ONLY returns the time in Melbourne Victoria
              Australia in four hours.
            </p>
            <p>
              Other API's give you the freedom to decide where and when we tell
              you where and when.
            </p>
          </div>
        </column>
      </row>
    </div>
    <div class="section features">
      <h2>Find the time in four hours in Melbourne!</h2>
      <p class="sub-heading">Free Time Forecast in JSON and Brainfuck!</p>
      <div>
        <row container :gutter="12">
          <column :xs="12" :md="4" :lg="3">
            <div class="grid-item">
              <img src="@/assets/tank.png" width="70" height="70" />
              <h3>Military Grade Time Data</h3>
              <p>
                No one in the Military has ever seen this but if they did they
                would probably be on board.
              </p>
            </div>
          </column>
          <column :xs="12" :md="4" :lg="3">
            <div class="grid-item">
              <img src="@/assets/now.png" width="70" height="70" />
              <h3>Open Source</h3>
              <p>
                Here at TIGTBIFHIMVA we understand that Software
                Artists(💻😔🖌️🎨) want open source. So at TIGTBIFHIMVA we take
                other open source projects slap a wrapper around it. Give it a
                fucking stupid name and release it into the world. Releasing a
                new version every 15 minutes breaking all previous versions and
                third party libraries.
              </p>
            </div>
          </column>
          <column :xs="12" :md="4" :lg="3">
            <div class="grid-item">
              <img src="@/assets/brainfuck.png" width="70" height="70" />
              <h3>Json and BrainFuck</h3>
              <p>
                All modern API's return json here at TIGTBIFHIMVA we understand
                that Software Artists(💻😔🖌️🎨) need BrainFuck to realise their
                pieces of art.
              </p>
            </div>
          </column>
          <column :xs="12" :md="4" :lg="3">
            <div class="grid-item">
              <img
                src="https://rustacean.net/assets/rustacean-flat-gesture.png"
                height="70"
              />
              <h3>Written in Rust</h3>
              <p>
                Everything needs to be written in Rust. The borrow checker is
                the return of christ. Real Software Artists(💻😔🖌️🎨) all use
                Rust for everything always. Those stupid gopher fuckers with if
                err != nil can fuck off. Fuck I love Rust.
              </p>
            </div>
          </column>
        </row>
      </div>
    </div>
    <div class="section">
      <h2>Example request</h2>
      <p>Get the time in Melbourne Victoria Australia four hours from now!</p>
      <div id="examples" :key="return_type">
        <button @click="toggleExample($event)">{{ button_text }}</button>
        <TIFH :return_type="return_type" />
      </div>
      <div class="codebase">
        <a
          href="https://discord.com/api/oauth2/authorize?client_id=736822440793210890&permissions=2147485696&scope=bot"
          >Check out the discord bot</a
        >
      </div>
      <div class="codebase">
        <a href="https://github.com/sardap/TIGTBIFHIMVA"
          >Check out the codebase📚</a
        >
      </div>
    </div>
    <div class="section features">
      <h2><a href="https://go.dev/">Fuck Rust is great</a></h2>
    </div>
    <div id="footer">
      <a href="mailto:support@tigtbifhimva.sarda.dev">
        <p>support@tigtbifhimva.sarda.dev</p>
      </a>
      <a href="https://sarda.dev/">
        <p>https://sarda.dev/</p>
      </a>
      <p>
        In case you somehow missed it this is a joke about how software
        exaggerates its importance. The old world isn't dead it's alive and
        kicking!.
      </p>
    </div>
  </div>
</template>

<style>
body {
  background-color: #ffffff;
}

.title {
  font-weight: bold;
  font-size: 40px;
}

.cn {
  display: flex;
  justify-content: center;
  align-items: center;
}

.codebase {
  padding: 10px;
}

#intro {
  background-color: #ffffff;
}

#footer {
  padding: 10px;
}

#examples {
  margin: 5px;
}

#examples button {
  margin: 20px;
  background-color: #f7d472;
  border: none;
  color: black;
  padding: 15px 32px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  -moz-box-shadow: 0 0 3px #ccc;
  -webkit-box-shadow: 0 0 3px #ccc;
  box-shadow: 0 0 3px #ccc;
}

.sub-heading {
  color: #828d9f;
  margin-block-start: -10px;
}

.section {
  padding-block-start: 30px;
  padding-block-end: 15px;
  padding-left: 20%;
  padding-right: 20%;
}

.section-top {
  padding-block-start: 0px;
  padding-block-end: 15px;
  padding-left: 20%;
  padding-right: 20%;
}

.grid-item {
  background-color: #ffffff;
  padding-left: 20px;
  padding-right: 20px;
  padding-top: 10px;
  padding-bottom: 10px;
  -moz-box-shadow: 0 0 3px #ccc;
  -webkit-box-shadow: 0 0 3px #ccc;
  box-shadow: 0 0 3px #ccc;
}

.grid-item p {
  text-align: left;
  margin-left: 10px;
}

.grid-item h3 {
  margin: 0px;
}

.features {
  background-color: #fef9eb;
}

.get-started {
  background-color: #fef9eb;
}
</style>

<script>
// @ is an alias to /src
import TIFH from "@/components/TIFH.vue";
import { Row, Column } from "vue-grid-responsive";

export default {
  name: "Home",
  components: {
    TIFH,
    Row,
    Column,
  },
  data: function () {
    return {
      show_json: true,
      return_type: "json",
      button_text: "show brainfuck",
    };
  },
  methods: {
    toggleExample: function () {
      if (this.return_type == "json") {
        this.return_type = "brainfuck";
        this.button_text = "show json";
      } else {
        this.return_type = "json";
        this.button_text = "show brainfuck";
      }
    },
  },
};
</script>
