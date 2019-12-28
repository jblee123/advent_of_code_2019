use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Layer {
    pub digit_counts: [u32; 10],
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
}

pub fn decode_image(data_str: &str, width: u32, height: u32) -> Image {
    let data = data_str.as_bytes();
    let pix_in_image = width * height;
    let data_len: u32 = data.len().try_into().unwrap();
    assert_eq!(data_len % pix_in_image, 0);
    let num_layers = data_len / pix_in_image;

    let mut img = Image {
        width: width,
        height: height,
        layers: vec![],
    };

    img.layers = (0..num_layers).map(|layer_num| {
        let start = layer_num * pix_in_image;
        let end = start + pix_in_image;
        let mut layer = Layer { digit_counts: [0; 10]};
        for byte in &data[start as usize..end as usize] {
            layer.digit_counts[(byte - b'0') as usize] += 1;
        }
        layer
    })
    .collect();

    img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_image() {
        let input = "123456789012";
        let result = decode_image(input, 3, 2);
        assert_eq!(result, Image {
            width: 3,
            height: 2,
            layers: vec![
                Layer { digit_counts: [0, 1, 1, 1, 1, 1, 1, 0, 0, 0] },
                Layer { digit_counts: [1, 1, 1, 0, 0, 0, 0, 1, 1, 1] },
            ],
        });
    }
}
