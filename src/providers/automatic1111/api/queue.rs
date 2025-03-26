//! Send image generation tasks to the queue.

use super::{Automatic1111Provider, txt2img::Txt2ImgRequestBody};
use crate::{
    images::{LvmImage, LvmImageMetadata},
    parameters::text_to_image::TextToImageRequest,
};
use anyhow::{Result, anyhow};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Debug, Deserialize)]
struct QueueTaskResponse {
    task_id: TaskId,
}

type TaskId = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TaskStatus {
    Pending,
    Running,
    Done,
    Failed,
    Interrupted,
}

#[derive(Debug, Serialize, Clone)]
struct OverrideSettings {}

/// Request body for starting a new image generation task.
#[derive(Debug, Serialize, Default, Clone)]
struct QueueRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg_scale: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_settings: Option<OverrideSettings>,
    pub script_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vae: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

/// Convert an f64 to a serde_json::Number.
/// Used to convert cfg_scale to a Number.
/// If the conversion fails, prints a warning and returns None.
fn cfg_scale_to_number(f: f64) -> Option<Number> {
    Number::from_f64(f).or_else(|| {
        eprintln!("Warning: Could not convert cfg_scale to Number.");
        None
    })
}

impl From<TextToImageRequest> for QueueRequestBody {
    fn from(request: TextToImageRequest) -> Self {
        let mut queue_request = QueueRequestBody {
            prompt: request.prompt.positive_prompt,
            negative_prompt: request.prompt.negative_prompt,
            width: request.width.map(|w| w.into()),
            height: request.height.map(|h| h.into()),
            ..Default::default()
        };
        if let Some(extended_params) = request.extended {
            queue_request.batch_size = extended_params.batch_size.map(|bs| bs.into());
            queue_request.steps = extended_params.steps.map(|s| s.into());
            queue_request.sampler_name = extended_params.sampler_name;
            queue_request.cfg_scale = extended_params.cfg_scale.and_then(cfg_scale_to_number);
            queue_request.checkpoint = extended_params.checkpoint;
            queue_request.vae = extended_params.vae;
            queue_request.callback_url = extended_params.callback_url;
        }
        queue_request
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskStatusParams {
    checkpoint: Option<String>,
    images: Vec<String>,
    info: String,
    parameters: Txt2ImgRequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskStatusResponse {
    success: bool,
    data: TaskStatusData,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskStatusData {
    id: String,
    api_task_id: Option<String>,
    api_task_callback: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    request_type: String,
    status: TaskStatus,
    params: Value,
    priority: Number,
    position: Option<Number>,
    result: Option<String>,
    bookmarked: Option<bool>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskResults {
    success: bool,
    data: Vec<TaskResultsData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskResultsData {
    /// The image in base64 encoding.
    image: String,
    /// The generation parameters for the image.
    infotext: String,
}

impl Automatic1111Provider {
    /// Send a POST request to `/agent-scheduler/v1/queue/txt2img` to start a new image generation task.
    /// The response contains the task_id for the image generation task.
    async fn start_image_generation_task(&self, request_body: &QueueRequestBody) -> Result<TaskId> {
        let endpoint = "/agent-scheduler/v1/queue/txt2img";
        let url = format!("{}{}", self.base_url, endpoint);
        let body = serde_json::to_string(request_body)?;
        let client = reqwest::Client::new();
        let response = client.post(url).body(body).send().await?;
        let response_text = response.text().await?;
        let response: QueueTaskResponse = serde_json::from_str(&response_text)?;
        Ok(response.task_id)
    }

    /// Check the status of the task.
    async fn get_task_status(&self, task_id: &TaskId) -> Result<TaskStatus> {
        let endpoint = format!("/agent-scheduler/v1/task/{}", task_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let response = reqwest::get(url).await?;
        let response_text = response.text().await?;
        let response: TaskStatusResponse = serde_json::from_str(&response_text)?;
        // The status is in the "msg" field of the response.
        let status = response.data.status;
        Ok(status)
    }

    /// Decode a base64-encoded image.
    fn decode_image(&self, image: &str) -> Result<Vec<u8>> {
        // The image string is prefixed with "data:image/png;base64," which needs to be removed.
        let image = image.trim_start_matches("data:image/png;base64,");
        let image = base64::prelude::BASE64_STANDARD.decode(image)?;
        Ok(image)
    }

    /// Get the results of the task. Results are a base64-encoded image.
    async fn get_task_results(&self, task_id: &str) -> Result<Vec<Vec<u8>>> {
        let endpoint = format!("/agent-scheduler/v1/task/{}/results", task_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let response = reqwest::get(url).await?;
        let response_text = response.text().await?;
        let results: TaskResults = serde_json::from_str(&response_text)?;
        let images: Vec<Vec<u8>> = results
            .data
            .iter()
            .map(|data| {
                let image = data.image.as_str();
                self.decode_image(image)
            })
            .collect::<Result<Vec<Vec<u8>>>>()?;
        Ok(images)
    }

    /// Poll the task until it is complete, returning the base64-encoded images.
    async fn poll_task(&self, task_id: &TaskId) -> Result<Vec<LvmImage>> {
        let timeout = std::time::Duration::from_secs(300);
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let status = self.get_task_status(task_id).await?;
                    match status {
                        TaskStatus::Done => {
                            let images: Vec<LvmImage> = self.get_task_results(task_id).await?
                                .iter()
                                .map(|image| {
                                    let metadata = Some(LvmImageMetadata {
                                        generation_params: serde_json::to_string(&Txt2ImgRequestBody::default()).ok(),
                                    });
                                    LvmImage {
                                        data: image.clone(),
                                        metadata,
                                    }
                                })
                                .collect();
                            return Ok(images);
                        }
                        TaskStatus::Failed => {
                            return Err(anyhow!("Task failed."));
                        }
                        _ => {}
                    }
                }
                _ = tokio::time::sleep(timeout) => {
                    return Err(anyhow!("Task timed out."));
                }
            }
        }
    }

    /// Send txt2img tasks to the queue for each num_batches.
    pub async fn queue_txt2img(&self, request: TextToImageRequest) -> Result<Vec<LvmImage>> {
        // If num_batches is None, default to 1.
        let num_batches = request.num_batches.unwrap_or(1);

        // Convert the request to a QueueRequestBody.
        let request = QueueRequestBody::from(request);

        // Send the requests to the queue and get the task_ids.
        let provider_config = std::sync::Arc::new(self.clone());
        let handles = (0..num_batches).map(|_| {
            let provider_config = std::sync::Arc::clone(&provider_config);
            let request_clone = request.clone();
            tokio::spawn(async move {
                provider_config
                    .start_image_generation_task(&request_clone)
                    .await
            })
        });
        let mut task_ids: Vec<Result<TaskId>> = Vec::new();
        for handle in handles {
            task_ids.push(handle.await?);
        }

        // Poll the tasks until they are complete.
        let mut images = Vec::new();
        for task_id in task_ids {
            match task_id {
                Ok(task_id) => {
                    let task_images = self.poll_task(&task_id).await?;
                    images.extend(task_images);
                }
                Err(e) => {
                    eprintln!("Error starting task: {}", e);
                }
            }
        }
        Ok(images)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial(stable_diffusion, local_server)]
    async fn test_start_image_generation_task() -> Result<()> {
        let provider = Automatic1111Provider::default();
        let request_body = QueueRequestBody::default();
        let task_id = provider.start_image_generation_task(&request_body).await?;
        // Assert that we get a task_id.
        assert!(!task_id.is_empty());
        Ok(())
    }

    #[tokio::test]
    #[serial(stable_diffusion, local_server)]
    async fn test_poll_task() -> Result<()> {
        let provider = Automatic1111Provider::default();
        let request_body = QueueRequestBody::default();
        let task_id = provider.start_image_generation_task(&request_body).await?;
        let image = provider.poll_task(&task_id).await?;
        assert!(!image.first().unwrap().data.is_empty());
        Ok(())
    }
}
