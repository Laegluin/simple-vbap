extern crate libc;
use libc::*;
use std::ptr::null;
use std::ffi::CString;

#[derive(Debug)]
pub struct LibVlc
{
    instance_ptr: intptr_t,
}

impl LibVlc 
{
    pub fn new() -> Option<LibVlc>
    {
        unsafe
        {
            let instance_ptr = libvlc_new(0, null());
            
            return if instance_ptr != 0
            {
                Option::Some(
                LibVlc
                {
                    instance_ptr: instance_ptr,
                })
            }
            else
            {
                Option::None
            }
        }
    }
}

impl Drop for LibVlc 
{
    fn drop(&mut self)
    {
        unsafe
        {
            libvlc_release(self.instance_ptr);
        }
    }
}

#[derive(Debug)]
pub struct Media
{
    media_ptr: intptr_t
}

impl Media 
{
    pub fn new(libvlc: &LibVlc, path: &str) -> Option<Media>
    {
        unsafe
        {
            let c_string = CString::from_vec_unchecked(path.as_bytes().to_vec());

            let media_ptr = libvlc_media_new_path(libvlc.instance_ptr, c_string.as_ptr());

            if media_ptr != 0
            {
                Option::Some(
                    Media
                    {
                        media_ptr: media_ptr,
                    })
            }
        else
        {
            Option::None
        }    
        }
    }

    pub fn new_empty() -> Media
    {
        Media
        {
            media_ptr: 0,
        }
    }
}

impl Drop for Media
{
    fn drop(&mut self)
    {
        unsafe
        {
            libvlc_media_release(self.media_ptr);
        }
    }
}

#[derive(Debug)]
pub struct MediaPlayer
{
    pub current_media: Media,
    player_ptr: intptr_t,    
}

impl MediaPlayer 
{
    pub fn new(libvlc: &LibVlc) -> Option<MediaPlayer>
    {
        unsafe
        {
            let player_ptr = libvlc_media_player_new(libvlc.instance_ptr);
            
            return if player_ptr != 0
            {
                Option::Some(
                MediaPlayer
                {
                    current_media: Media::new_empty(),
                    player_ptr: player_ptr,
                })
            }
            else
            {
                Option::None
            }
        }
    }

    pub fn set_media(&mut self, media: Media)
    {
        unsafe
        {
            self.current_media = media;
            libvlc_media_player_set_media(self.player_ptr, self.current_media.media_ptr);
        }
    }

    pub fn play(&self)
    {
        unsafe
        {
            libvlc_media_player_play(self.player_ptr);
        }
    }

    pub fn pause(&self)
    {
        unsafe
        {
            libvlc_media_player_set_pause(self.player_ptr, 1);
        }
    }
}

impl Drop for MediaPlayer 
{
    fn drop(&mut self)
    {
        unsafe
        {
            libvlc_media_player_release(self.player_ptr);
        }
    }
}


#[link(name="libvlc", kind="dylib")]
extern "C"
{
    fn libvlc_new(argc: c_int, args: *const *mut c_char) -> intptr_t;

    fn libvlc_release(instance_ptr: intptr_t);

    fn libvlc_media_player_new(libvlc_ptr: intptr_t) -> intptr_t;

    fn libvlc_media_player_release(player_ptr: intptr_t);

    fn libvlc_media_player_set_media(player_ptr: intptr_t, media_ptr: intptr_t);

    fn libvlc_media_player_play(player_ptr: intptr_t);

    fn libvlc_media_new_path(libvlc_ptr: intptr_t, path: *const c_char) -> intptr_t;

    fn libvlc_media_release(media_ptr: intptr_t);

    fn libvlc_media_player_set_pause(player_ptr: intptr_t, pause_flag: int32_t);
}