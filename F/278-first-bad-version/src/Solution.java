class VersionControl {
  public boolean isBadVersion(int version) {
    return true;
  }
}

public class Solution extends VersionControl {
  public int firstBadVersion(int n) {
    int start = 1;
    int end = n;
    for (; start < end; ) {
      int v = start + (end - start) / 2;
      if (this.isBadVersion(v)) {
        end = v;
      } else {
        start = v + 1;
      }
    }
    return end;
  }
}
