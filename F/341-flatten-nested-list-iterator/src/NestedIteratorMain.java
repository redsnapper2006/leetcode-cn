import java.util.Arrays;
import java.util.List;

public class NestedIteratorMain {

  public static void main(String[] args) {
    // [[1, 1],2,[1, 1]]
    NestedInteger first = new NestedIntegerImpl(false, 0,
                                                Arrays.asList(new NestedIntegerImpl(true, 1, null),
                                                              new NestedIntegerImpl(true, 1,
                                                                                    null)));
    NestedInteger second = new NestedIntegerImpl(true, 2, null);
    NestedInteger third = new NestedIntegerImpl(false, 0,
                                                Arrays.asList(new NestedIntegerImpl(true, 1, null),
                                                              new NestedIntegerImpl(true, 1,
                                                                                    null)));
    List<NestedInteger> root = Arrays.asList(first, second, third);

    //    NestedInteger first = new NestedIntegerImpl(false, 0, new ArrayList<NestedInteger>());
    //    List<NestedInteger> root = Arrays.asList(second, new NestedIntegerImpl(false, 0, Arrays.asList()));

    //    List<NestedInteger> root =
    //      Arrays.asList(new NestedIntegerImpl(false, 0, Arrays.asList()),
    //                    new NestedIntegerImpl(false, 0, Arrays.asList()), second,
    //                    new NestedIntegerImpl(false, 0, Arrays.asList()));

    NestedIterator ni = new NestedIterator(root);

    while (ni.hasNext()) {
      System.out.println(ni.next());
    }
  }
}
