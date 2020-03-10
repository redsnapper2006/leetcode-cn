public class FooBar {
  private int n;
  private final Object lock = new Object();
  private boolean wait;

  public FooBar(int n) {
    this.n = n;
    this.wait = false;
  }

  public void foo(Runnable printFoo) throws InterruptedException {

    for (int i = 0; i < n; i++) {
      synchronized (lock) {
        if (this.wait) {
          lock.wait();
        }
        // printFoo.run() outputs "foo". Do not change or remove this line.
        printFoo.run();
        this.wait = true;
        lock.notifyAll();
      }
    }
  }

  public void bar(Runnable printBar) throws InterruptedException {

    for (int i = 0; i < n; i++) {
      synchronized (lock) {
        if (!this.wait) {
          lock.wait();
        }
        // printBar.run() outputs "bar". Do not change or remove this line.
        printBar.run();
        this.wait = false;
        lock.notifyAll();
      }
    }
  }

  public static void main(String[] args) {
    FooBar fb = new FooBar(10);
    try {
      new Thread(new Runnable() {
        public void run() {
          try {
            fb.foo(new Runnable() {
              @Override
              public void run() {
                System.out.println("foo");
              }
            });
          } catch (Exception e) {
            e.printStackTrace();
          }
        }
      }).start();
      new Thread(new Runnable() {
        public void run() {
          try {
            fb.bar(new Runnable() {
              @Override
              public void run() {
                System.out.println("bar");
              }
            });
          } catch (Exception e) {
            e.printStackTrace();
          }
        }
      }).start();
    } catch (Exception e) {
      e.printStackTrace();
    }
  }
}
