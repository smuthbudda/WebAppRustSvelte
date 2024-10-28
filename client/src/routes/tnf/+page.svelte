<script lang="ts">
    export let data;
    import type { Gender, TrackPoints } from "$lib/types";
    import { APIClient } from "$lib/ApiClient";
    import { OutdoorEvents } from "$lib/const.js";
    import Card from "../../components/card.svelte";
    import {Button} from "@sveltestrap/sveltestrap";
    import { page } from "$app/stores";
    import { onMount } from 'svelte';

    let apiclient = new APIClient();
    let loading: boolean = false;
    $: cookie = data.cookie;

    let myEvents: TrackPoints[] = [];
    let category = "Outdoor";
    let gender = "Male";
    let event = "100m";
    let time: number;

    let eventsList: TrackPoints[] = [];

    onMount(async () => {
        try {
            myEvents = await apiclient.GetMyPoints(cookie, $page.data.user.id)
            eventsList = [...eventsList, ...myEvents];
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    });

    async function getResults() {
        let result = await apiclient.getResults(category, gender, event, time);
        if (result) {
            console.log(result)
            if (eventsList.filter((x) => result.Id == x.Id).length == 0){
                eventsList = [...eventsList, result];
            }
        }
    }

    function userHasPoints(id: number): boolean{
        return myEvents.filter((x) => x.Id == id).length > 0
    }

    async function loadDataToDB() {
        loading = true;
        await apiclient.loadDataToDB();
        loading = false;
    }

    async function addPointsToUser(object: TrackPoints) {
        await apiclient.requestUserPoints(cookie, $page.data.user.id, object.Id, "POST");
        myEvents = [...myEvents, object];
        eventsList = [...eventsList];
    }

    async function deleteUserPoint(points_id: number){
        await apiclient.requestUserPoints(cookie, $page.data.user.id, points_id, "DELETE");
        eventsList = eventsList.filter((x) => x.Id != points_id);
    }

    let GenderArr = ["Male", "Female"];
    let CategoryArr = ["Indoor", "Outdoor"];
    const authorizedExtensions = ['.jpg', '.jpeg', '.png', '.webp'];
</script>

<Card title="World Athletics Points Conversion" footer="">
    <div>
        <select bind:value={category}>
            {#each CategoryArr as cat}
                <option value={cat}>
                    {cat}
                </option>
            {/each}
        </select>
        <select bind:value={gender}>
            {#each GenderArr as gen}
                <option value={gen}>
                    {gen}
                </option>
            {/each}
        </select>
        <select bind:value={event}>
            {#each OutdoorEvents as ev}
                <option value={ev.Event}>
                    {ev.Event}
                </option>
            {/each}
        </select>
        <input bind:value={time} placeholder="Time" />
        <button on:click={getResults}>Submit</button>
    </div>

    <table class="styled-table">
        <thead>
            <tr>
                <th scope="row">Category</th>
                <th>Event</th>
                <th>Gender</th>
                <th>Mark</th>
                <th>Points</th>
                <th>Add</th>
            </tr>
        </thead>
        <tbody>
            {#if eventsList.length > 0}
                {#each eventsList as row}
                    <tr>
                        <td>{row.Category}</td>
                        <td>{row.Event}</td>
                        <td>{row.Gender}</td>
                        <td>{row.Mark}</td>
                        <td>{row.Points}</td>
                        <td>
                            {#if userHasPoints(row.Id)}
                                <Button on:click={async() => await deleteUserPoint(row.Id)} type="button">Delete</Button>
                            {:else}
                                <Button on:click={async() => await addPointsToUser(row)} type="button">Add</Button>
                            {/if}
                        </td>
                    </tr>
                {/each}
            {/if}
        </tbody>
    </table>

<!--    <div>-->
<!--        {#if loading}-->
<!--            <small>Loading</small>-->
<!--        {:else}-->
<!--            <button-->
<!--                on:click={async () => {-->
<!--                    await loadDataToDB();-->
<!--                }}>Load Data</button-->
<!--            >-->
<!--        {/if}-->
<!--    </div>-->
</Card>


<style>
    .styled-table {
        border-collapse: collapse;
        font-size: 0.9em;
        min-width: 400px;
        width: 100%;
    }

    .styled-table thead tr {
        background-color: #009879;
        color: #ffffff;
        text-align: left;
    }

    .styled-table th,
    .styled-table td {
        padding: 12px 15px;
    }

    .styled-table tbody tr {
        border-bottom: 1px solid #dddddd;
    }

    .styled-table tbody tr:nth-of-type(even) {
        background-color: #f3f3f3;
    }

    .styled-table tbody tr:last-of-type {
        border-bottom: 2px solid #009879;
    }
</style>
