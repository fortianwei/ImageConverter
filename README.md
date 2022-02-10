两个模块:
1、 VsConverter 负责读取 文件夹/文件信息，调用ImageMagick进行转换
2、 Register   
    申请管理员权限执行后续步骤
    检查Magick.exe是否存在，没有的话安装。
    写入注册表，添加右键选项
    调用VsConverter

Register.exe中包含VsConverter.exe和ImageMagick.exe，前者放到C:/Windows/下
，后者在第一次运行的时候放到TMP目录然后执行安装。
