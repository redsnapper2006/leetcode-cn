import java.util.*;

public class NestedIterator implements Iterator<Integer> {
  private Stack<ListIterator<NestedInteger>> stack;

  public NestedIterator(List<NestedInteger> nestedList) {
    this.stack = new Stack<ListIterator<NestedInteger>>();
    this.stack.push(nestedList.listIterator());
  }

  @Override
  public Integer next() {
    ListIterator<NestedInteger> li = this.stack.pop();
    NestedInteger cur = li.next();

    if (li.hasNext()) {
      this.stack.push(li);
    }

    if (cur.isInteger()) {
      return cur.getInteger();
    } else {
      do {
        li = cur.getList().listIterator();

        if (li.hasNext()) {
          cur = li.next();
        } else {
          cur = null;
        }
        this.stack.push(li);
      } while ((cur != null) && !cur.isInteger());
      return cur.getInteger();
    }
  }

  @Override
  public boolean hasNext() {
    while (!this.stack.isEmpty()) {
      ListIterator<NestedInteger> li = this.stack.pop();
      while (li.hasNext()) {
        NestedInteger first = li.next();
        if (li.hasNext()) {
          this.stack.push(li);
        }
        if (first.isInteger()) {
          this.stack.push(Arrays.asList(first).listIterator());
          return true;
        } else {
          li = first.getList().listIterator();
        }
      }
    }
    return false;
  }
}


