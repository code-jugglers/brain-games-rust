export async function main() {
  console.log("APP STARTING");

  const train_btn = document.getElementById("train");
  const results_container = document.getElementById("results");

  const worker = await createGameWorker();

  train_btn.addEventListener("click", async () => {
    results_container.innerHTML = "Training...";

    const training_results = await worker.train();

    results_container.innerHTML = training_results.replaceAll("\n", "<br>");
  });
}

function createGameWorker() {
  return new Promise((resolve) => {
    const worker = new Worker("js/game.worker.js", { type: "module" });

    worker.addEventListener("message", (msg) => {
      if (msg.data.status === "READY") {
        resolve({
          train() {
            return new Promise((resolve) => {
              function listen(msg) {
                if (msg.data.status === "TRAINING_COMPLETE") {
                  worker.removeEventListener("message", listen);

                  resolve(msg.data.message);
                }
              }

              worker.addEventListener("message", listen);
              worker.postMessage({ action: "TRAIN" });
            });
          },
        });
      }
    });
  });
}
