# API design and encapsulation

## Key loop

Get the SurfaceTexture -> Perform a render pass over SurfaceTexture -> Render

## Structs and their dependents

## Essentials

Instance -> Adapter

Adapter -> Device + Queue

Device -> Encoder

### To render

**CommandBuffer** -> Queue -> *Render* ***wow***

### To CommandBuffer

RenderPassDescriptor -> CommandEncoder -> RenderPass + **shenanigins** -> finish -> *CommandBuffer* ***:O***

set_bind_group + set_pipeline + set_index_buffer + set_vertex_buffer + draw_indexed -> *CommandBuffer*

### Pipeline

**Device + RenderPipelineDescriptor** -> RenderPipeline -> *set_pipeline*

PipelineLayout + VertexState + PrimitiveState + \<DepthStensilState\> + MultiSampleState + FragmentState -> RenderPipelineDescriptor

### Vertex

### Bindgroup

### Texture
