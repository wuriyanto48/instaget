use serde::{ Serialize, Deserialize };

pub struct DownloadData {
    pub download_url: String,
    pub is_video: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    graphql: Option<GraphQl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQl {
    shortcode_media: Option<GraphMedia>,
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMedia {
    __typename: Option<String>,
    shortcode: Option<String>,
    dimensions: Option<MediaDimension>,
    is_video: Option<bool>,
    display_url: Option<String>,
    video_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDimension {
    height: u64,
    width: u64,
}

impl GraphMedia {
    fn download_url(&self) -> Result<DownloadData, String> {
        if self.is_video.as_ref().is_none() {
            return Err(String::from("error: is video empty"));
        }

        if self.is_video.unwrap() {
            if self.video_url.as_ref().is_none() {
                return Err(String::from("error: video url empty"));
            }

            return Ok(DownloadData {
                download_url: self.video_url.as_ref().unwrap().clone(),
                is_video: true
            });
        }

        if self.display_url.is_none() {
            return Err(String::from("error: display url empty"));
        }

        Ok(DownloadData {
            download_url: self.display_url.as_ref().unwrap().clone(),
            is_video: false
        })

    } 
}

impl GraphQl {
    fn download_url(&self) -> Result<DownloadData, String> {
        if self.shortcode_media.is_none() {
            return Err(String::from("error: shortcode_media empty"));
        }

        if let Err(err) = self.shortcode_media.as_ref().unwrap().download_url() {
            return Err(err);
        }

        Ok(self.shortcode_media.as_ref().unwrap().download_url().unwrap())
    }
}

impl GraphData {
    pub fn download_url(&self) -> Result<DownloadData, String> {
        if self.graphql.is_none() {
            return Err(String::from("error: graphql empty"));
        }

        if let Err(err) = self.graphql.as_ref().unwrap().download_url() {
            return Err(err);
        }

        Ok(self.graphql.as_ref().unwrap().download_url().unwrap())
    }
}