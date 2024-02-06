import * as wasm from "majin-blob-wasm";

document
  .getElementById("blobForm")
  .addEventListener("submit", async function (event) {
    event.preventDefault(); // Prevent the form from submitting in the traditional way

    const fileInput = document.getElementById("blobFileInput");
    const textInput = document.getElementById("blobTextInput");
    let blob;

    if (fileInput.files.length > 0) {
      // Read the blob from the file
      const file = fileInput.files[0];
      blob = await file.text();
    } else {
      // Use the manually entered blob data
      blob = textInput.value;
    }
    const state_diffs = wasm.blob_recover(blob);

    // Format the JSON string to be pretty
    const formattedStateDiffs = JSON.stringify(
      JSON.parse(state_diffs),
      null,
      2,
    );

    document.getElementById("stateDiffsOutput").textContent =
      formattedStateDiffs;
  });
