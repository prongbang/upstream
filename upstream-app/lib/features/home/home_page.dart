import 'dart:io';

import 'package:dio/dio.dart';

import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:percent_indicator/percent_indicator.dart';
import 'package:upstream/core/filex/file_x_utility.dart';
import 'package:upstream/features/scanner/scanner_page.dart';

class FileX {
  File file;
  double progress;
  String status;

  FileX({required this.file, required this.progress, required this.status});
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final String _ip = '192.168.0.18:5000';
  late FileXType _selectedType;
  List<FileXType> _fileTypes = [];
  final List<FileX> _files = [];

  final _fileXUtility = GetIt.I.get<FileXUtility>();

  @override
  void initState() {
    _fileTypes = _fileXUtility.getTypeList();
    _selectedType = _fileTypes[0];
    super.initState();
  }

  void _selectFile(FileXType type) async {
    var result = await _fileXUtility.pickFiles(type);

    if (result != null) {
      _files.clear();

      List<File> files = result.paths.map((path) => File(path!)).toList();
      for (File f in files) {
        _files.add(FileX(file: f, progress: 0.0, status: 'IN-PROGRESS'));
      }
      setState(() {});
      for (int i = 0; i < _files.length; i++) {
        await _uploadFile(i, _files[i].file, onProgress: (index, progress) {
          setState(() {
            _files[i].progress = progress;
            _files[i].status = progress >= 100 ? 'DONE' : 'PROGRESS';
          });
        });
      }
    }
  }

  Future<void> _uploadFile(
    int index,
    File? selectedFile, {
    required Function(int index, double progress) onProgress,
  }) async {
    if (selectedFile == null) return;

    String uploadUrl = 'http://$_ip/up';
    Dio dio = Dio();

    try {
      FormData formData = FormData.fromMap({
        'file': await MultipartFile.fromFile(selectedFile.path),
      });

      final response = await dio.post(
        uploadUrl,
        data: formData,
        onSendProgress: (int sent, int total) {
          double progress = (sent / total) * 100;
          debugPrint("Upload progress: $progress%");
          onProgress(index, progress);
        },
      );

      if (response.statusCode == 200) {
        debugPrint('File uploaded successfully: ${response.data}');
      } else {
        debugPrint('Upload failed: ${response.statusCode}');
      }
    } catch (e) {
      debugPrint('Error: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: const Text('UPSTREAM'),
      ),
      body: SafeArea(
        child: Padding(
          padding:
              const EdgeInsets.only(top: 16, left: 24, right: 24, bottom: 24),
          child: Column(
            children: [
              Card(
                child: Row(
                  children: [
                    Expanded(
                      child: Padding(
                        padding: const EdgeInsets.all(16.0),
                        child: Text(
                          'Paired: $_ip',
                          style: const TextStyle(color: Colors.green),
                        ),
                      ),
                    ),
                    IconButton(
                      tooltip: 'Scan to pair',
                      onPressed: () {
                        _navigateToQrScanner();
                      },
                      icon: const Icon(Icons.qr_code_scanner_rounded),
                    )
                  ],
                ),
              ),
              const SizedBox(height: 16),
              Expanded(
                child: ListView.builder(
                  itemCount: _files.length,
                  itemBuilder: (context, index) {
                    final file = _files[index];
                    return Card(
                      child: Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: Row(
                          children: [
                            Expanded(
                              child: Column(
                                children: [
                                  Text(file.file.path),
                                ],
                              ),
                            ),
                            CircularPercentIndicator(
                              radius: 20.0,
                              lineWidth: 5.0,
                              animation: true,
                              percent: file.progress / 100,
                              circularStrokeCap: CircularStrokeCap.round,
                              progressColor: Colors.green,
                            ),
                          ],
                        ),
                      ),
                    );
                  },
                ),
              ),
              Column(
                children: [
                  DropdownButton<FileXType>(
                    isExpanded: true,
                    value: _selectedType,
                    hint: const Text("Select file type"),
                    items: _fileTypes.map((fileType) {
                      return DropdownMenuItem<FileXType>(
                        value: fileType,
                        child: Text(fileType.name),
                      );
                    }).toList(),
                    onChanged: (FileXType? value) {
                      setState(() {
                        _selectedType = value!;
                      });
                    },
                  ),
                  const SizedBox(height: 16),
                  SizedBox(
                    width: double.infinity,
                    child: ElevatedButton(
                      onPressed: () => _selectFile(_selectedType),
                      child: Container(
                        height: 55,
                        alignment: Alignment.center,
                        child: const Text('Select File'),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  void _navigateToQrScanner() async {
    Navigator.push(
      context,
      MaterialPageRoute(builder: (context) => const ScannerPage()),
    ).then((value) {
      print('QR: $value');
    });
  }
}
