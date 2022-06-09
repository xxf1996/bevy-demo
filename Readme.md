- https://bevyengine.org/learn/book/getting-started/setup/

## 常用命令

- 编译为wasm并运行指定example：

  ```bash
  cargo run --example obj_custom --target wasm32-unknown-unknown
  ```

  更多基于bevy编译为wasm的细节可以参考：[Browser (WebAssembly) - Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/platforms/wasm.html)

- 一般运行指定example：

  ```bash
  cargo run --example obj_custom -F bevy/dynamic
  ```

  由于wasm32不支持dynamic特征，所以在cargo.toml中就没有直接开启，因此可以在命令通过`--features`开启，进行编译加速（有没有办法配置不同的target使用不同的features？）；

- 







## 注意事项

- 在Mac端安装zld的时候（`brew install michaeleisel/zld/zld`），可能会出现如下报错：

  ```bash
  xcode-select: error: tool 'xcodebuild' requires Xcode, but active developer directory '/Library/Developer/CommandLineTools' is a command line tools instance
  ```

  可以考虑使用如下命令来解决[^1]：

  ```bash
  sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer/
  ```

- gltf加载出现如下错误时：

  ```bash
  WARN bevy_gltf::loader: Error loading glTF texture: You may need to add the feature for the file format: failed to load an image: The image format Jpeg is not supported
  ```

  需要手动开启bevy的jpeg feature[^2]：

  ```toml
  [dependencies]
  bevy = { version = "0.7.0", features = ["jpeg"] }
  ```

  

- 开启bevy的dynamic feature后，编译wasm会出现报错：

  ![image-20220609151452684](http://pic.xiexuefeng.cc/markdown/image-20220609151452684.png?imageslim)

  原因是开启这个特征后bevy会使用bevy_dylib这个库；可以关闭这个特征，编译wasm就会成功，但是**编译速度很慢**。目前在网上找到一个说法是dynamic特征只适合于桌面端target[^3]，因此wasm32应该不包含在内？

- 



## 素材来源

- [realistic interior](https://www.cgtrader.com/free-3d-models/interior/bedroom/realistic-bedroom-ef83a980-44c8-4046-a179-c37b582e2bff)

### 候补场景

- [Cozy Room free 3D model | CGTrader](https://www.cgtrader.com/free-3d-models/interior/living-room/cozy-room-98cce965-ef86-4d82-b4cd-0010be9a533b)
- [Indivi Sofa Replica free 3D model | CGTrader](https://www.cgtrader.com/free-3d-models/interior/living-room/indivi-sofa-replica)
- [Architectural Interior Scene - Living Room free 3D model | CGTrader](https://www.cgtrader.com/free-3d-models/interior/living-room/architectural-interior-scene-living-room)

[^1]: [xcode-select: error: tool 'xcodebuild' requires Xcode - 简书](https://www.jianshu.com/p/07a281ff57d3)
[^2]: https://github.com/bevyengine/bevy/issues/1391#issuecomment-773428852
[^3]: https://bevy-cheatbook.github.io/setup/bevy-config.html?highlight=dynamic#dynamic-linking
