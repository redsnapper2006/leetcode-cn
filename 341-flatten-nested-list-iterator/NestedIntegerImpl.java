import java.util.List;

public class NestedIntegerImpl implements NestedInteger {
  private boolean isInteger;
  private Integer i;
  private List<NestedInteger> l;

  public NestedIntegerImpl(boolean isInteger, Integer i, List<NestedInteger> l) {
    this.isInteger = isInteger;
    this.i = i;
    this.l = l;
  }

  public boolean isInteger() {
    return isInteger;
  }

  public Integer getInteger() {
    return i;
  }

  public List<NestedInteger> getList() {
    return l;
  }
}
