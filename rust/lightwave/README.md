# LightWave 3D Rust Parser

## LightWave Object (LWO2)

Progress: About 90% ([LWO2 Spec](http://static.lightwave3d.com/sdk/2015/html/filefmts/lwo2.html))

| Chunk                                      | Tag    | Status |
|--------------------------------------------|--------|--------|
| Layer                                      | `LAYR` | ✅      |
| Point List                                 | `PNTS` | ✅      |
| Vertex Mapping                             | `VMAP` | ✅      |
| Polygon List                               | `POLS` | ✅      |
| Tag Strings                                | `TAGS` | ✅      |
| Polygon Tag Mapping                        | `PTAG` | ✅      |
| Discontinuous Vertex Mapping               | `VMAD` | ✅      |
| Vertex Map Parameter                       | `VMPA` | ✅      |
| [Envelope Definition](#envelope-subchunks) | `ENVL` | ❌      |
| [Image (-Sequence)](#clip-subchunks)       | `CLIP` | 🚧     |
| [Surface Definition](#surface-subchunks)   | `SURF` | 🚧     |
| Bounding Box                               | `BBOX` | ✅      |
| Description Line                           | `DESC` | ❌      |
| Commentary Text                            | `TEXT` | ❌      |
| Thumbnail Icon Image                       | `ICON` | ❌      |


### Envelope Subchunks

| Chunk                    | Tag    | Status |
|--------------------------|--------|--------|
| Envelope Type            | `TYPE` | ❌      |
| Pre-Behavior             | `PRE`  | ❌      |
| Post-Behavior            | `POST` | ❌      |
| Keyframe Time and Value  | `KEY`  | ❌      |
| Interval Interpolation   | `SPAN` | ❌      |
| Plugin Channel Modifiers | `CHAN` | ❌      |
| Channel Name             | `NAME` | ❌      |


### Clip Subchunks

| Chunk                | Tag    | Status |
|----------------------|--------|--------|
| Still Image          | `STIL` | ✅      |
| Image Sequence       | `ISEQ` | ✅      |
| Plugin Animation     | `ANIM` | ❌      |
| Reference (Clone)    | `XREF` | ✅      |
| Flag (Undocumented)  | `FLAG` | ⚠️     | 
| Color-cycling Still  | `STCC` | ✅      |
| Time                 | `TIME` | ✅      |
| Color Space RGB      | `CLRS` | ✅      |
| Color Space Alpha    | `CLRA` | ✅      |
| Image Filtering      | `FILT` | ✅      |
| Image Dithering      | `DITH` | ✅      |
| Contrast             | `CONT` | ✅      | 
| Brightness           | `BRIT` | ✅      | 
| Saturation           | `SATR` | ✅      | 
| Hue                  | `HUE`  | ✅      | 
| Gamma Correction     | `GAMM` | ✅      | 
| Negative             | `NEGA` | ❌      | 
| Plugin Image Filters | `IFLT` | ❌      | 
| Plugin Pixel Filters | `PFLT` | ❌      | 


### Surface Subchunks

### Basic Surface Parameters

| Chunk                             | Tag                                                      | Status |
|-----------------------------------|----------------------------------------------------------|--------|
| Base Color                        | `COLR`                                                   | ✅      |
| Base Shading Values               | `DIFF`<br>`LUMI`<br>`SPEC`<br>`REFL`<br>`TRAN`<br>`TRNL` | ✅      |
| Specular Glossiness               | `GLOS`                                                   | ✅      |
| Diffuse Sharpness                 | `SHRP`                                                   | ✅      |
| Bump Intensity                    | `BUMP`                                                   | ✅      | 
| Polygon Sidedness                 | `SIDE`                                                   | ✅      |
| Max Smoothing Angle               | `SMAN`                                                   | ✅      |
| Reflection Options                | `RFOP`                                                   | ✅      |
| Reflection Map Image              | `RIMG`                                                   | ✅      |
| Reflection Map Image Seam Angle   | `RSAN`                                                   | ✅      |
| Reflection Blurring               | `RBLR`                                                   | ✅      |
| Refractive Index                  | `RIND`                                                   | ✅      | 
| Transparency Options              | `TROP`                                                   | ✅      | 
| Refraction Map Image              | `TIMG`                                                   | ✅      | 
| Refraction Blurring               | `TBLR`                                                   | ✅      | 
| Color Highlights                  | `CLRH`                                                   | ✅      | 
| Color Filter                      | `CLRF`                                                   | ✅      | 
| Additive Transparency             | `ADRT`                                                   | ✅      | 
| Glow Effect                       | `GLOW`                                                   | ✅      | 
| Render Outlines                   | `LINE`                                                   | ✅      | 
| Alpha Mode                        | `ALPH`                                                   | ✅      | 
| Vertex Color Map                  | `VCOL`                                                   | ✅      | 
| [Surface Blocks](#surface-blocks) | `BLOK`                                                   | 🚧     | 

### Surface Blocks

Ordinal Strings:
* ✅ [Image Texture Map](#image-texture-map) `IMAP`
* ❌ [Procedural Texture](#procedural-texture) `PROC`
* ❌ [Gradient Texture](#gradient-texture) `GRAD`
* ❌ Shader Plugin `SHDR`

#### Shared

| Chunk                   | Tag    | Status |
|-------------------------|--------|--------|
| Texture Channel         | `CHAN` | ✅      |
| Enable State            | `ENAB` | ✅      |
| Opacity                 | `OPAC` | ✅      |
| Displacement Axis       | `AXIS` | ✅      |
| Negative (Undocumented) | `NEGA` | ⚠️     |

#### Texture Mapping

| Chunk               | Tag                        | Status |
|---------------------|----------------------------|--------|
| Positioning         | `CNTR`<br>`SIZE`<br>`ROTA` | ✅      |
| Reference Object    | `OREF`                     | ✅      |
| Falloff             | `FALL`                     | ✅      |
| Coordinate System   | `CSYS`                     | ✅      | 

#### Image Texture Map

| Chunk                               | Tag              | Status |
|-------------------------------------|------------------|--------|
| [Texture Mapping](#texture-mapping) | `TMAP`           | ✅      |
| Projection Mode                     | `PROJ`           | ✅      |
| Major Axis                          | `AXIS`           | ✅      |
| Image Map                           | `IMAG`           | ✅      |
| Image Wrap Options                  | `WRAP`           | ✅      |
| Image Wrap Amount                   | `WRPW`<br>`WRPH` | ✅      | 
| UV Vertex Map                       | `VMAP`           | ✅      |
| Antialiasing Strength               | `AAST`           | ✅      |
| Pixel Blending                      | `PIXB`           | ✅      |
| Sticky Projection                   | `STCK`           | ✅      |
| Texture Ampliture                   | `TAMP`           | ✅      |

#### Procedural Texture

| Chunk                    | Tag    | Status |
|--------------------------|--------|--------|
| Axis                     | `AXIS` | ❌      |
| Basic Value              | `VALU` | ❌      |
| Algorithm and Parameters | `FUNC` | ❌      |

#### Gradient Texture

| Chunk          | Tag               | Status |
|----------------|-------------------|--------|
| Parameter Name | `PNAM`            | ❌      |
| Item Name      | `INAM`            | ❌      |
| Gradient Range | `GRST`<br>`GREN`  | ❌      |
| Repeat Mode    | `GRPT`            | ❌      |
| Key Values     | `FKEY`            | ❌      |
| Key Parameters | `IKEY`            | ❌      |

#### Shaders


| Chunk            | Tag    | Status |
|------------------|--------|--------|
| Shader Algorithm | `FUNC` | ❌      |
