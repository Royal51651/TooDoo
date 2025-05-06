<script>
  import { page } from "$app/state";
    import { hide } from "@tauri-apps/api/app";
  import { invoke } from "@tauri-apps/api/core";

  /**
   * TODO
   * Add a settings menu
   * TODO
   * Use cookies, or something in rust to store notes, and load them
  */

  /**
     * @type {any[]}
     */
  let groups = $state([]);
  let input_note = $state("");
  let id_count = 0;

  let announcer_message = $state("Default Announcer Message");
  let announcer_state = $state("hidden");
  let blocker_state = $state("hidden");
  // yn in this context stands for yes / no, I swear
  let announcer_yn_state = $state("hidden");
  let announcer_yn_callback = $state(() => {});

  let color_mode = $state("random");
  let page_count = $state(0);
  let notes_per_page = $state(3);
  let slice = $derived(groups.slice(page_count, page_count + notes_per_page));
  let save_frequency = $state(4);
  let save_count = $state(0);
  
  $effect(() => {
    console.log(save_count);
  });

  const randomColor = () => {
      let color = "";
      for(let i = 0; i < 3; i++){
          color += (Math.floor(Math.random() * 64) + 96).toString() + ",";
      }
      return color;
  }

  /**
   * 
   * @param mode - specifies for a random color, black, or white
   * @returns {string} - returns a string representing an rgb value in the form red,green,blue,
   */
  const get_color = (/** @type {string} */ mode) => {
    if(mode == "random"){
      return randomColor();
    } else if(mode == "black"){
      return "0,0,0,";
    } else if (mode == "white"){
      return "255,255,255,";
    }
    // this should hopefully never run
    return "Something went wrong :(";
  }

  /**
   * 
   * @param input - the input string to parse into a valid css color style
   * @returns {string} - returns a string in the form of rgb(0 - 255, 0 - 255, 0 - 255);
   */
  const parseColor = (/** @type {string} */ input) => {
    if(input == "Something went wrong :("){
      console.log(input);
      throw "ahhhh";
    }
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

  /**
   * 
   * @param group_name - the name that the group will have when added
   */
  const add_group = (/** @type {any} */ group_name) => {
    if(group_name != ""){
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
      save();
    }
  }

  const add_note = (/** @type {number} */ id) => {
    if(groups[id].temp_msg != ""){
      const note = {
        text: groups[id].temp_msg,
        color: parseColor(get_color(color_mode)),
        finished: "Done",
        id: groups[id].id_counter,
      }
      groups[id].data.push(note);
      groups[id].temp_msg = "";
      groups[id].id_counter += 1;
      save();
    }
  }

  async function save(){
    save_count++;
    if(save_count >= save_frequency){
      let output = "";
      let group_count = 0;
      for(let group of groups){
        output += `add_group("${group.name}");`;
        for(let note of group.data){
          output += `add_note_for_save("${note.text}", "${note.color}", "${note.finished}", ${group_count});`;
        }
        group_count++;
      }
      await invoke("save", {
        data: output,
      });
      save_count = 0;
    }
  }

  async function load(/** @type {string}*/ input_name){
    groups = [];
    let result = await invoke("load", {
      name: input_name,
    });
    eval(result);
    save_count = 0;
  }

  // @ts-ignore
  const add_note_for_save = (message, input_color, finished_status, id) => {
    const note = {
      text: message,
      color: input_color,
      finished: finished_status,
      id: groups[id].id_counter,
    }
    groups[id].data.push(note);
    groups[id].id_counter += 1;
  }

  const remove_group = (/** @type {number} */ id) => {
    for(let i = 0; i < groups.length; i++){
      if(groups[i].id == id){
        groups.splice(i, 1);
        break;
      }
    }
    save();
  }

  const remove_note = (/** @type {any} */ input, /** @type {number} */ id) => {
    let dat = groups[id].data;
    for(let i = 0; i < dat.length; i++){
      if(dat[i].id == input){
        groups[id].data.splice(i, 1);
        break;
      }
    }
    save();
  }

  const finish_note = (/** @type {any} */ input, /** @type {number} */ id) => {
    for(let i = 0; i < groups[id].data.length; i++){
      if(groups[id].data[i].id == input){
        if (groups[id].data[i].finished == "Undo"){
          remove_note(input, id);
        } else {
          groups[id].data[i].finished ="Undo";
          let item = groups[id].data[i];
          groups[id].data.splice(i, 1);
          groups[id].data.push(item);
        }
        
        break;
      }
    }
    save();
  }

  const announce = (/** @type {string} */ msg) => {
    announcer_message = msg;
    announcer_state = "";
    blocker_state = "";
  }

  const announce_yn = (/** @type {string} */ msg) => {
    announcer_message = msg;
    announcer_yn_state = "";
    blocker_state = "";
  }

  const hide_announcer = () => {
    announcer_state = "hidden";
    announcer_yn_state = "hidden";
    blocker_state = "hidden";
  }

  const scroll = (/** @type {number} */ dir) => {
    console.log(Math.floor(groups.length / notes_per_page));
    if(dir == -1 && page_count != 0){
      page_count -= 1;
    } else if (dir == 1 && page_count < groups.length - notes_per_page){
      page_count += 1;
    }
  }

  function delete_single_group(/** @type {number} */ group_id) {
    announcer_yn_callback = () => {
      remove_group(group_id);
      for(let i = group_id; i < groups.length; i++){
        groups[i].id--;
      }
      hide_announcer();
    }
    announce_yn("Are you sure you want to delete this group?");
  }

  load("save.txt");

</script>

<!--  This area is outside the main div because the announcer should cover the entire screen properly </!-->

<div class="blocker {blocker_state}"></div>

<div class="announcer" style="visibility: {announcer_state}">
  <span>
    <p1>{announcer_message}</p1>
  </span>

  <span>
    <button onclick={hide_announcer}>Okay</button>
  </span>
  
</div>

<div class="announcer" style="visibility: {announcer_yn_state}">
  <span>
    <p1>{announcer_message}</p1>
  </span>

  <span>
    <button onclick={announcer_yn_callback}>Yes</button>
    <button onclick={hide_announcer}>No</button>
  </span>
  
</div>

<div class="main">

  <div class="container">
    {#if page_count != 0}
    <span style="width: 10%;">
      <button onclick={() => scroll(-1)}>&lt;</button>
    </span>
    {/if}
    <span style="width: 100%;">
      <input

      type="text"
      onkeydown={(e) => e.key === "Enter" && add_group(input_note)}
      placeholder="Add Group"
      bind:value={input_note}

    >
    </span>
    {#if page_count < groups.length - notes_per_page}
    <span style="width: 10%;">
      <button onclick={() => scroll(1)}>&gt;</button>
    </span>
    {/if}
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

        <button onclick={() => finish_note(item.id, group.id)} class="item {item.finished}" style="background-color: {item.color};">
            {item.text}
        </button>

        {/each}

        <button onclick={() => delete_single_group(group.id)} class="item">
          Delete Group {group.name}
        </button>
      </div>
    {/each}
  </div>

</div>



<style>

:root {
  
  --main-color: #ffffff;
  --sub-color: #000000;

  line-height: 24px;
  font-weight: 400;
  box-sizing: border-box;
  color: var(--main-color);
  background-color: var(--sub-color);
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  overflow: none;
  -ms-overflow-style: none;
  scrollbar-width: none;

}

.main {
  margin: 4px;
}

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

.container button {
  background-color: var(--main-color);
  color: var(--sub-color);
  width: 100%;
  height: 100%;
}

.blocker {
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.8);
  position: fixed;
  top: 1;
  bottom: 0;
  z-index: 50;
}

.announcer p1, .announcer button {
  font-size: min(3vw, 3vh);
  color: var(--sub-color);
}

.announcer button {
  color: var(--sub-color);
  background-color: var(--main-color);
}

.hidden {
  visibility: hidden;
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
  border: 2px dashed var(--main-color);
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 5px;
  margin: 5px;
}

.item:hover {
  border: 1px solid (--main-color);
}

.item {
  width: 95%;
  border: 1px solid transparent;
  justify-content: center;
  align-items: center;
  display: flex;
  margin: 10px;
  border-radius: 5px;
  padding: 10px;
  box-sizing: border-box;
  overflow-wrap: anywhere;
  cursor: pointer;
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
  border: 2px solid var(--main-color);
  padding: 20px;
  color: #000000;
  box-sizing: border-box;
}

.group button {
  width: 95%;
}

button {
  padding: 5px;
  background-color: var(--main-color);
  color: var(--sub-color);
  border: 2px solid transparent;
  border-radius: 5px;
}

*{
  margin: 0;
  padding: 0;
  font-size: 20px;
  font-family: Poppins, sans-serif;
}

</style>
