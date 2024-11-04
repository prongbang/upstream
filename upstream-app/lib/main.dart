import 'dart:io';

import 'package:dio/dio.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'UPSTREAM',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blueAccent),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String _progress = '';

  void _selectFile() async {
    var result = await FilePicker.platform.pickFiles(allowMultiple: true);

    if (result != null) {
      List<File> files = result.paths.map((path) => File(path!)).toList();
      for (File f in files) {
        await _uploadFile(context, f);
      }
    }
  }

  Future<void> _uploadFile(BuildContext context, File? selectedFile) async {
    if (selectedFile == null) return;

    const String uploadUrl = 'http://192.168.0.18:5000/up';
    Dio dio = Dio();

    try {
      FormData formData = FormData.fromMap({
        'file': await MultipartFile.fromFile(selectedFile.path),
      });

      final response = await dio.post(
        uploadUrl,
        data: formData,
        onSendProgress: (int sent, int total) {
          // Calculate the progress percentage
          double progress = (sent / total) * 100;
          _progress = progress.toStringAsFixed(2);
          debugPrint("Upload progress: ${_progress}%");
          setState(() {});
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
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text('Progress: $_progress'),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _selectFile,
              child: const Text('Select File'),
            ),
          ],
        ),
      ),
    );
  }
}
