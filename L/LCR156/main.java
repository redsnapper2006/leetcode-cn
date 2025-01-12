import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;

  TreeNode(int x) {
    val = x;
  }
}

public class Codec {

  public static void main(String[] args) {
    Codec c = new Codec();
    TreeNode t = c.deserialize("1,2,3,null,null,null,4,null,null");
    System.out.println(t.val);
    String s = c.serialize(t);
    System.out.println(s);
  }

  private String rserialize(TreeNode r, String s) {
    if (r == null) {
      return s + "null,";
    }
    s += Integer.toString(r.val) + ",";
    s = rserialize(r.left, s);
    s = rserialize(r.right, s);
    return s;
  }

  public String serialize(TreeNode root) {
    return rserialize(root, "");
  }

  private TreeNode rdeserialize(List<String> l) {
    String s = l.get(0);
    l.remove(0);
    if (s.equals("null")) {
      return null;
    }
    TreeNode tn = new TreeNode(Integer.parseInt(s));
    tn.left = rdeserialize(l);
    tn.right = rdeserialize(l);
    return tn;
  }

  public TreeNode deserialize(String data) {
    String[] buf = data.split(",");
    return rdeserialize(new ArrayList<String>(Arrays.asList(buf)));
  }
}
