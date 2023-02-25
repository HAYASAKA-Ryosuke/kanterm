<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from 'svelte';

  let readlineString = "";
  let textarea: HTMLElement;
  let baudRate: number = 9600;
  let newline: string = "\n";
  let paths: string = "";
  let selectedPath: string = "";
  let isOpened: boolean = false;
  let timer: NodeJS.Timer
  let sendingText: string = "";
  let binaryModeEnabled: boolean = false;

  async function connect() {
    isOpened = true;
    await invoke("open_port", {path: selectedPath, baudRate: baudRate});
    console.log(binaryModeEnabled);
    if (binaryModeEnabled) {
      console.log('binary');
      timer = setInterval(() => {
        binaryModeReads()
	    }, 100);
    } else {
      timer = setInterval(() => {
        asciiModeReads()
	    }, 100);
    }
  }
  async function disconnect() {
    isOpened = false;
    clearInterval(timer);
    await invoke("close_port");
  }
  onMount(async () => {
    textarea = document.getElementById('textareaId') as HTMLElement;
    paths = await invoke("fetch_ports");
    if (paths.length > 0) {
      selectedPath = paths[0]
    }
  });

  async function asciiModeReads() {
    let result: string = await invoke("readlines");
    readlineString += result.replaceAll('\r', '\\r').replaceAll('\n', '\\n').replaceAll(newline, '\n');
    textarea.scrollTop = textarea.scrollHeight;
  }
  async function binaryModeReads() {
    let result: string = await invoke("readlines");
    readlineString += (new TextEncoder).encode(result).toString().replaceAll(',', ' ');
    textarea.scrollTop = textarea.scrollHeight;
  }
  function changePort(event: any) {
    selectedPath = event.target.value;
  }
  function changeBaudRate(event: any) {
    baudRate = event.target.value;
  }
  function changeNewline(event: any) {
    newline = event.target.value;
  }
  async function sendText() {
    console.log(sendingText);
    await invoke('write', {data: sendingText});
    sendingText = '';
  }
</script>

<div class="row">
  <input disabled={!isOpened} type="text" bind:value={sendingText} />
  <button disabled={!isOpened} on:click={sendText}>send</button>
</div>

<div class="row">
  <textarea id="textareaId" rows=30 cols=100 bind:value={readlineString}/>
</div>
<div class="row">
  <input disabled={isOpened} type="checkbox" bind:checked={binaryModeEnabled}>
  <label for="binaryMode">binary mode</label>
</div>

<div class="row">
 	<select disabled={isOpened} on:change={changePort}>
	{#each paths as path}
	  <option value={path}>
	    {path}
    </option>
	{/each}
	</select>
  <select disabled={isOpened} on:change={changeBaudRate}>
    <option value=9600 selected>9600 bps</option>
    <option value=38400>38400 bps</option>
  </select>
  <select disabled={isOpened} on:change={changeNewline}>
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
