use euclid::{Size2D, Rect, Box2D, Point2D};

use crate::{
    binding::Bind, device::Device, transform::ScreenSpace, color::{Color}
};

#[derive(Debug)]
/// A texture
pub struct Texture {
    /// Wrapped wgpu value
    pub wgpu: wgpu::Texture,
    /// A texture view generated by the texture
    pub view: wgpu::TextureView,
    pub extent: wgpu::Extent3d,
    /// Format of texture
    pub format: wgpu::TextureFormat,
    /// Size of the texture
    pub size: Size2D<u32, ScreenSpace>
}

impl Texture {
    /// Clears a texture with a singular color
    pub fn clear<T> (
        texture: &Texture,
        colour: T,
        device: &Device
    ) where 
    T: Color + Clone + Copy,
    {
        let capacity = texture.size.to_usize().area();
        let mut t_pixels : Vec<T> = Vec::with_capacity(capacity);
        t_pixels.resize(capacity, colour);
        
        Self::fill(
            texture,
            t_pixels.as_slice(),
            &device,
            
        )
    }
    
    /// Fill a texture with texture pixels
    pub fn fill<T> (
        texture: &Texture,
        t_pixels: &[T],
        device: &Device
    ) where
    T: bytemuck::Pod + Clone + Copy + 'static + Color,
    {
        assert!(
            t_pixels.len() as u32 >= texture.size.area(),
            "Fatal: incorrect length for t_pixel buffer. Pixels length: {} || Required buffer length: {}", t_pixels.len(), texture.size.area()
        );
        
        let rect = Rect::from_size(texture.size);
        let t_pixels = bytemuck::cast_slice(t_pixels);
        
        Self::copy(
            texture,
            rect,
            &device.queue,
            t_pixels,
            t_pixels.len() as u32 / texture.extent.height,
            texture.extent
        )
    }
    
    /// Transfer texture pixels to a rect in a texture
    pub fn transfer<T>(
        texture: &Texture,
        t_pixels: &[T],
        rect: Rect<u32, ScreenSpace>,
        device: &Device,
    ) where
    T: bytemuck::Pod + Clone + Copy + 'static,
    {
        let destination = rect.to_box2d();
        
        // Make sure we have a positive rectangle
        let destination: Box2D<u32, ScreenSpace> = Box2D::new (
            Point2D::new(
                destination.min.x.min(destination.max.x),
                destination.min.y.min(destination.max.y)
            ),
            Point2D::new(
                destination.max.x.max(destination.min.x),
                destination.max.y.max(destination.max.y),
            )
        );
        
        // Flip y axis as wgpu has its texture y axis pointing down
        let destination: Box2D<u32, ScreenSpace> = Box2D::new(
            Point2D::new(destination.min.x, destination.max.y),
            Point2D::new(destination.max.x, destination.min.y)
        );
        
        let rect = destination.to_rect();
        
        // Width and height of transfer area
        let destination_size = rect.size;
        
        // The destination coordinate of the transfer on the texture.
        // We have to invert the y coordingate as explained above
        let destination_point = Point2D::new(
            rect.origin.x,
            texture.size.height - rect.origin.y
        );
        
        assert!(
            destination_size.area() <= texture.size.area(),
            "Fatal: transfer size must be <= to the texture size"
        );
        
        let t_pixels: &[u8] = bytemuck::cast_slice(t_pixels);
        
        let extent = wgpu::Extent3d {
            width: destination_size.width,
            height: destination_size.height,
            depth_or_array_layers: 0,
        };
        Self::copy(
            &texture,
            Rect::new(destination_point, destination_size),
            &device.queue,
            t_pixels,
            t_pixels.len() as u32 / texture.extent.height * 4 as u32,
            extent
        )
    }
    
    /// ...
    pub fn blit(
        &self,
        src: Rect<u32, ScreenSpace>,
        dst: Rect<u32, ScreenSpace>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        assert!(
            src.area() == dst.area(),
            "Source and destination rectangles must be of the same size"
        );
        
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &self.wgpu,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: src.origin.x,
                    y: src.origin.y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &self.wgpu,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: dst.origin.x,
                    y: dst.origin.y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: src.width(),
                height: src.height(),
                depth_or_array_layers: 0,
            }
        )
    }
    
    fn copy (
        texture: &Texture,
        desitination: euclid::Rect<u32, ScreenSpace>,
        queue: &wgpu::Queue,
        t_pixels: &[u8],
        bytes_per_row: u32,
        extent: wgpu::Extent3d,
    ) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture.wgpu,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: desitination.origin.x,
                    y: desitination.origin.y,
                    z: 0
                },
                aspect: wgpu::TextureAspect::All
            },
            t_pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(bytes_per_row),
                rows_per_image: std::num::NonZeroU32::new(desitination.size.height),
            },
            extent, 
        )
    }
}

impl Bind for Texture {
    fn binding(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index,
            resource: wgpu::BindingResource::TextureView(&self.view)
        }
    }
}
