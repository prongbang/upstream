class PercentUtility {
  String calc(int sent, int total) {
    double progress = (sent / total) * 100;
    return progress.toStringAsFixed(2);
  }
}
