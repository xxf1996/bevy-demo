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

  

- 

[^1]: [xcode-select: error: tool 'xcodebuild' requires Xcode - 简书](https://www.jianshu.com/p/07a281ff57d3)
