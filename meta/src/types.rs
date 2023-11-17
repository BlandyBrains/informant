pub fn image_types() -> Vec<&'static str> {
    return vec![
        "heic",
        "heif",
        "jpeg",
        "jpg",
        "png",
        "raf",  // Fuji Raw Format
        "tif",
        "tiff",
        "cr2",
        "jfif", // legacy
        /*
        GRAPHICS
        "cgm", 
        "svg",
        "gif",
        "bmp",
        "bpg",
        "pbm",
        "pgm",
        "pnm",
        */
    ];
}

pub fn video_types() -> Vec<&'static str> {
    return vec![
        "3g2",
        "3gp",
        "3gp",
        "3gpp",
        "amv",
        "asf",
        "avi",
        "drc",
        "f4a",
        "f4b",
        "f4p",
        "f4v",
        "flv",
        "m2v",
        "m4p",
        "m4v",
        "mkv",
        "mng",
        "mov",
        "mp2",
        "mp4",
        "mpe",
        "mpeg",
        "mpg",
        "mts",
        "mxf",
        "nsv",
        "ogg",
        "ogv",
        "qt",
        "rm",
        "rmvb",
        "svi",
        "vob",
        "webm",
        "wmv",
        "yuv",
    ];
}

pub fn audio_types() -> Vec<&'static str> {
    return vec![
        "mp3",
        "wav",
        "flac",
        "amr",
        "m4a",
        "wma",
        "m4r"
    ];
}


#[cfg(test)]
mod test {
    use crate::types::{image_types, video_types};

    fn find(dataset: Vec<&str>, e: &str) -> bool {
        for x in dataset.into_iter() {
            if e.to_lowercase().ends_with(&x.to_lowercase()){
                return true;
            }
        }
        return false;
    }

    #[test]
    fn test_find() {
        assert_eq!(false, find(image_types(), ""));
        assert_eq!(true, find(image_types(), "/some/file.heic"));
        assert_eq!(false, find(image_types(), "/some/other/file.heic2"));
        assert_eq!(true, find(video_types(), "/some/other/file.rm"));
    }
}