<script lang="ts">
    import Card from "../../components/card.svelte";

    export let data;
</script>

<Card title="Update {data.first_name} Profile" footer="{data.last_name}">
    <form method="post">
        <div class="input-group">
            <label for="first_name">First Name:</label>
            <input
                type="text"
                id="first_name"
                name="first_name"
                bind:value={data.first_name}
                required
            />
        </div>
        <div class="input-group">
            <label for="last_name">Last Name:</label>
            <input
                type="text"
                id="last_name"
                name="last_name"
                bind:value={data.last_name}
                required
            />
        </div>
        <div class="input-group">
            <label for="email">Email:</label>
            <input
                type="email"
                id="email"
                name="email"
                bind:value={data.email}
                required
            />
        </div>
        <div class="input-group">
            <label for="phone">Phone:</label>
            <input
                type="tel"
                id="phone"
                name="phone"
                bind:value={data.phone}
                required
            />
        </div>
        <div class="button-container">
            <button type="submit">Submit</button>
        </div>
    </form>
</Card>

<style>
    form {
        display: flex;
        flex-direction: column;

    }

    .input-group {
        display: grid;
        grid-template-columns: 1fr 2fr; /* Adjusted column sizes */
        gap: 0.5rem;
        margin-bottom: 1rem;
        border-bottom: 1px solid #dddddd; /* Border to match table row */
        padding-bottom: 0.5rem;
    }

    label {
        font-weight: bold;
        color: #333; /* Slightly darker text for contrast */
    }

    input {
        padding: 0.5rem;
        border: 1px solid #dddddd; /* Border to match table cell */
        border-radius: 4px; /* Rounded corners for input fields */
        background: #f9f9f9; /* Light background for input fields */
    }

    .button-container {
        display: flex;
        justify-content: flex-end;
    }

    button {
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 4px;
        background-color: #009879; /* Matching background color */
        color: white;
        font-size: 1rem;
        cursor: pointer;
    }

    button:hover {
        background-color: #007961; /* Slightly darker on hover */
    }
</style>
