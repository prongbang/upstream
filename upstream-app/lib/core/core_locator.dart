import 'package:file_picker/file_picker.dart';
import 'package:get_it/get_it.dart';
import 'package:get_storage/get_storage.dart';
import 'package:upstream/core/filex/file_x_utility.dart';
import 'package:upstream/core/percent/percent_utility.dart';

class CoreLocator {
  static void register() {
    GetIt.I.registerFactory(() => FilePicker.platform);
    GetIt.I.registerFactory(() => PercentUtility());
    GetIt.I.registerFactory<FileXUtility>(
      () => LocalFileXUtility(GetIt.I.get<FilePicker>()),
    );
    GetIt.I.registerFactory(() => GetStorage('upstream'));
  }
}
