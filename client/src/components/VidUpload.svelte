<script lang="ts">

    const authorizedExtensions = [".jpg", ".jpeg", ".png", ".webp"];
</script>

<section>
    <form method="post" enctype="multipart/form-data">
        <div class="group">
            <label for="file">Upload the video file</label>
            <input
                type="file"
                id="file"
                name="fileToUpload"
                accept={authorizedExtensions.join(",")}
                required
            />
        </div>

        <button type="submit">Submit</button>
    </form>
</section>
