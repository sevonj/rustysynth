use std::error;
use std::io;

use super::binary_reader;
use super::soundfont_version::SoundFontVersion;

#[non_exhaustive]
pub struct SoundFontInfo
{
    pub version: SoundFontVersion,
    pub target_sound_engine: String,
    pub bank_name: String,
    pub rom_name: String,
    pub rom_version: SoundFontVersion,
    pub creation_date: String,
    pub author: String,
    pub target_product: String,
    pub copyright: String,
    pub comments: String,
    pub tools: String,
}

impl SoundFontInfo
{
    pub(crate) fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let result = binary_reader::read_four_cc(reader);
        let chunk_id = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if chunk_id != "LIST"
        {
            return Err(format!("The LIST chunk was not found.").into());
        }

        let result = binary_reader::read_i32(reader);
        let end = match result
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let mut pos: i32 = 0;

        let result = binary_reader::read_four_cc(reader);
        let list_type = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if list_type != "INFO"
        {
            return Err(format!("The type of the LIST chunk must be 'INFO', but was '{list_type}'.").into());
        }
        pos += 4;

        let mut version: Option<SoundFontVersion> = None;
        let mut target_sound_engine: Option<String> = None;
        let mut bank_name: Option<String> = None;
        let mut rom_name: Option<String> = None;
        let mut rom_version: Option<SoundFontVersion> = None;
        let mut creation_date: Option<String> = None;
        let mut author: Option<String> = None;
        let mut target_product: Option<String> = None;
        let mut copyright: Option<String> = None;
        let mut comments: Option<String> = None;
        let mut tools: Option<String> = None;

        while pos < end
        {
            let result = binary_reader::read_four_cc(reader);
            let id = match result
            {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            pos += 4;

            let result = binary_reader::read_i32(reader);
            let size = match result
            {
                Ok(value) => value,
                Err(error) => return Err(Box::new(error)),
            };
            pos += 4;

            if id == "ifil"
            {
                let result = SoundFontVersion::new(reader);
                version = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else if id == "isng"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                target_sound_engine = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "INAM"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                bank_name = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "irom"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                rom_name = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "iver"
            {
                let result = SoundFontVersion::new(reader);
                rom_version = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else if id == "ICRD"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                creation_date = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "IENG"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                author = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "IPRD"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                target_product = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "ICOP"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                copyright = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "ICMT"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                comments = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "ISFT"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                tools = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else
            {
                return Err(format!("The INFO list contains an unknown ID '{id}'.").into());
            }

            pos += size;
        }

        let version = match version
        {
            Some(value) => value,
            None => SoundFontVersion::default(),
        };

        let target_sound_engine = match target_sound_engine
        {
            Some(value) => value,
            None => String::new(),
        };

        let bank_name = match bank_name
        {
            Some(value) => value,
            None => String::new(),
        };

        let rom_name = match rom_name
        {
            Some(value) => value,
            None => String::new(),
        };

        let rom_version = match rom_version
        {
            Some(value) => value,
            None => SoundFontVersion::default(),
        };

        let creation_date = match creation_date
        {
            Some(value) => value,
            None => String::new(),
        };

        let author = match author
        {
            Some(value) => value,
            None => String::new(),
        };

        let target_product = match target_product
        {
            Some(value) => value,
            None => String::new(),
        };

        let copyright = match copyright
        {
            Some(value) => value,
            None => String::new(),
        };

        let comments = match comments
        {
            Some(value) => value,
            None => String::new(),
        };

        let tools = match tools
        {
            Some(value) => value,
            None => String::new(),
        };

        Ok(SoundFontInfo
        {
            version: version,
            target_sound_engine: target_sound_engine,
            bank_name: bank_name,
            rom_name: rom_name,
            rom_version: rom_version,
            creation_date: creation_date,
            author: author,
            target_product: target_product,
            copyright: copyright,
            comments: comments,
            tools: tools,
        })
    }
}
