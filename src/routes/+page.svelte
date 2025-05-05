<script>
    import { page } from "$app/state";
  import { invoke } from "@tauri-apps/api/core";

  /**
     * @type {any[]}
     */
  let groups = $state([]);
  let input_note = $state("");
  let id_count = 0;
  let announcer_message = $state("Default Announcer Message");
  let announcer_state = $state("hidden");
  let announcer_enabled = $state(true);
  let page_count = $state(0);
  let slice = $derived(groups.slice(page_count, page_count + 3));

  const randomColor = () => {
      let color = "";
      for(let i = 0; i < 3; i++){
          color += (Math.floor(Math.random() * 64) + 96).toString() + ",";
      }
      return color;
  }

  const parseColor = (/** @type {string | any[]} */ input) => {
      let output = "rgb(";
      let count = 0;
      for(let i = 0; i < input.length; i++){
          if(input[i] == ","){
              count += 1; 
          }
          if(count != 3){
              output += input[i];
          }
      }
      return output + ");";
  }

  const add_group = (/** @type {any} */ group_name) => {
    /**
       * @type {never[]}
       */
    let item_list = $state([]);
    const group = {
      name: group_name,
      data: item_list,
      temp_msg: "",
      id_counter: 0,
      id: id_count
    }
    input_note = "";
    id_count += 1;
    groups.push(group);
  }

  const add_note = (/** @type {number} */ id) => {
    if(groups[id].temp_msg != ""){
        const note = {
        text: groups[id].temp_msg,
        color: parseColor(randomColor()),
        finished: "Done",
        id: groups[id].id_counter,
      }
      groups[id].data.push(note);
      groups[id].temp_msg = "";
      groups[id].id_counter += 1;
    }
  }

  const remove_note = (/** @type {any} */ input, /** @type {number} */ id) => {
    let dat = groups[id].data;
    for(let i = 0; i < dat.length; i++){
      if(dat[i].id == input){
        groups[id].data.splice(i, 1);
        break;
      }
    }
  }

  const finish_note = (/** @type {any} */ input, /** @type {number} */ id) => {
    for(let i = 0; i < groups[id].data.length; i++){
      if(groups[id].data[i].id == input){
        if (groups[id].data[i].finished == "Undo"){
          groups[id].data[i].finished ="Done";
        } else {
          groups[id].data[i].finished ="Undo";
        }
        
        break;
      }
    }
  }

  const announce = (/** @type {string} */ msg) => {
    if(announcer_enabled){
      announcer_message = msg;
      announcer_state = "";
    }
  }

  const hide_announcer = () => {
    announcer_state = "hidden";
  }

  const disable_announcer = () => {
    announcer_enabled = false;
    announcer_state = "hidden";
  }
  const scroll = (/** @type {number} */ dir) => {
    console.log(Math.floor(groups.length / 3));
    if(dir == -1 && page_count != 0){
      page_count -= 1;
    } else if (dir == 1 && page_count < groups.length - 3){
      page_count += 1;
    }
  }

</script>

<div class="blocker" style="visibility: {announcer_state}"></div>

<div class="announcer" style="visibility: {announcer_state}">
  <span>
    <p1>{announcer_message}</p1>
  </span>

  <span>
    <button onclick={hide_announcer}>Okay</button>
    <button onclick={disable_announcer}>Disable Announcer</button>
  </span>
  
</div>

<div class="container">
  <button onclick={() => scroll(-1)}>&lt;</button>
  <span style="width: 80%;">
    <input

    type="text"
    onkeydown={(e) => e.key === "Enter" && add_group(input_note)}
    placeholder="Add Group"
    bind:value={input_note}

  >
  </span>
  <button onclick={() => scroll(1)}>&gt;</button>
</div>

<div class="noteContainer">
  {#each slice as group}
    <div class="group">

      <input

        type="text"
        onkeydown={(e) => e.key === "Enter" && add_note(group.id)}
        placeholder="Add To {group.name}"
        bind:value={group.temp_msg}

      >

      {#each group.data as item}

      <div class="item" style="background-color: {item.color};">
        <span style="width: 5%;">
          <button onclick={() => finish_note(item.id, group.id)}>{item.finished}</button>
        </span>
        <span style="width: 85%;">
          <p1 class="{item.finished}">{item.text}</p1>
        </span>
        <span style="width: 10%;">
          <button onclick={() => remove_note(item.id, group.id)}>Delete</button>
        </span> 
      </div>
    
      {/each}
    </div>
  {/each}
</div>

<style>

.announcer {
  width: 80vw;
  height: 50vh;
  justify-content: space-evenly;
  align-items: center;
  display: flex;
  border-radius: 5px;
  flex-direction: column;
  padding: 10px;
  box-sizing: border-box;
  background-color: rgb(108, 157, 125);
  position: fixed;
  top: 20vh;
  left: 10vw;
  z-index: 100;
}

.blocker {
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.9);
  position: fixed;
  top: 0;
  bottom: 0;
  z-index: 50;
}

.announcer p1, .announcer button {
  color: #ffffff;
  font-size: min(3vw, 3vh);
}

.announcer span {
  width: 100%;
  display: flex;
  justify-content: center;
  gap: 10px;
}

.container {
  justify-content: center;
  align-items: center;
  display: flex;
  width: 100%;
  height: 100%;
}

.noteContainer {
  justify-content: space-between;
  display: flex;
  width: 100%;
  height: 100%;
}

.group {
  border: 2px dashed #ffffff;
  width: 95%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 5px;
  margin: 5px;
}

.item {
  width: 95%;
  justify-content: center;
  align-items: center;
  display: flex;
  margin: 10px;
  border-radius: 5px;
  padding: 10px;
  box-sizing: border-box;
}

.Undo {
  text-decoration: line-through;
}

span{
  display: flex; 
  align-items: center; 
  justify-content: center;
  padding-left: 5px;
  padding-right: 5px;
  box-sizing: border-box;
}

input {
  width: 95%;
  justify-content: center;
  align-items: center;
  display: flex;
  margin: 10px;
  border-radius: 5px;
  border: none;
  padding: 20px;
  color: #000000;
  box-sizing: border-box;
}

button {
  padding: 5px;
  background-color: #000000;
  color: #ffffff;
  border: 2px solid transparent;
  border-radius: 10px;
}

*{
  margin: 0;
  padding: 0;
  font-size: 20px;
  font-family: Poppins, sans-serif;
}

:root {
  
  line-height: 24px;
  font-weight: 400;
  box-sizing: border-box;
  color: #ffffff;
  background-color: #000000;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  overflow: none;
  -ms-overflow-style: none;
  scrollbar-width: none;

}

</style>
