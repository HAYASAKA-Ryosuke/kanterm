<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from 'svelte';

  let readlineString = "";
  let textarea: HTMLElement;
  let baudRate: number = 9600;
  let paths: string = "";
  let selectedPath: string = "";
  let isOpened: boolean = false;
  let timer: NodeJS.Timer

  async function connect() {
    isOpened = true;
    await invoke("open_port", {path: selectedPath, baudRate: baudRate});
    timer = setInterval(() => {
      reads()
	  }, 100);
  }
  async function disconnect() {
    isOpened = false;
    clearInterval(timer);
    await invoke("close_port");
  }
  onMount(async () => {
    textarea = document.getElementById('textarea_id') as HTMLElement;
    paths = await invoke("fetch_ports");
    if (paths.length > 0) {
      selectedPath = paths[0]
    }
  });

  async function reads() {
    readlineString += await invoke("readlines");
    textarea.scrollTop = textarea.scrollHeight;
  }
  function changePort(event: any) {
    selectedPath = event.target.value;
  }
  function changeBaudRate(event: any) {
    baudRate = event.target.value;
  }
</script>


<div class="row">
  <textarea id="textarea_id" rows=50 cols=100 bind:value={readlineString}/>
</div>

<div class="row">

 	<select on:change={changePort}>
	{#each paths as path}
	  <option value={path}>
	    {path}
    </option>
	{/each}
	</select>
  <select id="bardRateId" on:change={changeBaudRate}>
    <option value=9600 selected>9600 bps</option>
    <option value=38400>38400 bps</option>
  </select>
  <select id="newlineid">
    <option value="\n" selected>LF</option>
    <option value="\r">CR</option>
    <option value="\r\n">CRLF</option>
  </select>
  <button type="button" on:click={isOpened ? disconnect : connect}>{ isOpened ? "disconnect" : "connect" }</button>
</div>


<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
