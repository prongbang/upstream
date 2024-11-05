import 'package:file_picker/file_picker.dart';

class FileXType {
  String name;
  FileType type;

  FileXType({required this.name, required this.type});
}

abstract class FileXUtility {
  List<FileXType> getTypeList();

  Future<FilePickerResult?> pickFiles(FileXType type);
}

class LocalFileXUtility implements FileXUtility {
  final FilePicker _filePicker;

  LocalFileXUtility(this._filePicker);

  @override
  List<FileXType> getTypeList() {
    return [
      FileXType(name: 'MEDIA', type: FileType.media),
      FileXType(name: 'IMAGE', type: FileType.image),
      FileXType(name: 'AUDIO', type: FileType.audio),
      FileXType(name: 'VIDEO', type: FileType.video),
      FileXType(name: 'ANY', type: FileType.any),
      FileXType(name: 'CUSTOM', type: FileType.custom),
    ];
  }

  @override
  Future<FilePickerResult?> pickFiles(FileXType type) {
    return _filePicker.pickFiles(type: type.type, allowMultiple: true);
  }
}
