<script lang="ts">
  import { baseAPIUrl, baseWSUrl } from "$lib/const";
  import type { SystemInfo } from "$lib/types";
  import { onMount } from "svelte";
  import { Spinner } from "@sveltestrap/sveltestrap";
  let socket;
  let url = `${baseWSUrl}/api/system/cpu`;
  let chart_data: number[] = [];
  let system_info: SystemInfo;

  onMount(async () => {
    await fetch(`${baseAPIUrl}/api/system/details`)
      .then((response) => response.json())
      .then((data) => {
        system_info = data.data;
      })
      .catch((error) => {
        console.error("Error:", error);
      });

    socket = new WebSocket(url);
    socket.onmessage = (event) => {
      const receivedData = JSON.parse(event.data);
      chart_data = receivedData;
    };
  });
</script>

<div class="system_container">
  <div class="card cpu">
    {#each chart_data as cpu}
      <div class="bar">
        <div class="bar-inner" style="width: {cpu}%"></div>
        <small>{cpu.toFixed(2)}%</small>
      </div>
    {/each}
    {#if chart_data.length == 0}
      <Spinner type="border" color="primary" />
    {/if}
  </div>
  <div class="card">
    <h4>System Information</h4>
    <section class="system_info">
      {#if system_info != undefined}
        <div><strong>CPU Name:</strong> {system_info.cpu_info.brand}</div>
        <div><strong>OS:</strong> {system_info.os}</div>
        <div>
          <strong>Total Ram:</strong>
          {system_info.ram_total / (1024 * 1024 * 10)}
        </div>
        <div><strong>CPU Name:</strong> {system_info.disks[0]}</div>
      {/if}
    </section>
  </div>
</div>

<style>
  .system_container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .card {
    background-color: var(--primary-card-color);
    padding: 1rem;
  }
  .cpu {
    display: flex;
    gap: 0.8rem;
    flex-direction: column;
    min-height: 500px;
  }
  .bar {
    background: #264653;
    color: white;

    width: 100%;
    height: 2em;

    display: flex;
    align-items: center;
    justify-content: center;

    position: relative;
  }

  .bar small {
    z-index: 10;
  }

  .bar .bar-inner {
    z-index: 5;

    height: 100%;

    background: #2a9d8f;
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;

    transition: width 0.2s;
  }

  .bar,
  .bar-inner {
    border-radius: 4px;
  }

  .system_info {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin: 2rem 1rem 0 0;
  }
</style>
