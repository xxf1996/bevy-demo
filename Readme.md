- https://bevyengine.org/learn/book/getting-started/setup/

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

  

- 



## 素材来源

- [realistic interior](https://www.cgtrader.com/free-3d-models/interior/bedroom/realistic-bedroom-ef83a980-44c8-4046-a179-c37b582e2bff)

### 候补场景

- [Cozy Room free 3D model | CGTrader](https://www.cgtrader.com/free-3d-models/interior/living-room/cozy-room-98cce965-ef86-4d82-b4cd-0010be9a533b)
- [Indivi Sofa Replica free 3D model | CGTrader](https://www.cgtrader.com/free-3d-models/interior/living-room/indivi-sofa-replica)
- [Architectural Interior Scene - Living Room free 3D model | CGTrader](https://www.cgtrader.com/free-3d-models/interior/living-room/architectural-interior-scene-living-room)

[^1]: [xcode-select: error: tool 'xcodebuild' requires Xcode - 简书](https://www.jianshu.com/p/07a281ff57d3)
[^2]: https://github.com/bevyengine/bevy/issues/1391#issuecomment-773428852
