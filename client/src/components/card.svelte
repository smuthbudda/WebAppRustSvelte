<script lang="ts">
    export let title: string;
    export let footer: string;
</script>

<div class="card">
    {#if title}
        <div class="card-header">
            <h2>{title}</h2>
        </div>
    {/if}
    <div class="card-body">
        <slot></slot> <!-- This is where the content will be inserted -->
    </div>
    {#if footer}
        <div class="card-footer">
            <p>{footer}</p>
        </div>
    {/if}
</div>

<style>
    .card {
        background: white;
        border: 1px solid #dddddd;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        padding: 1rem;
        margin: 1rem;
    }

    .card-header {
        border-bottom: 1px solid #dddddd;
        padding-bottom: 0.5rem;
    }

    .card-body {
        flex: 1;
    }

    .card-footer {
        border-top: 1px solid #dddddd;
        padding-top: 0.5rem;
        text-align: right;
        font-size: 0.9em;
        color: #555;
    }

    h2 {
        margin: 0;
        font-size: 1.25rem;
    }
</style>
