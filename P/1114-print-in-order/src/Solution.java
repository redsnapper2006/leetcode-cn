import java.util.concurrent.Semaphore;

class Foo {
  private final Semaphore mutex2 = new Semaphore(1);
  private final Semaphore mutex3 = new Semaphore(1);

  public Foo() {
    try {
      mutex2.acquire();
      mutex3.acquire();
    } catch (InterruptedException e) {
      e.printStackTrace();
    }
  }

  public void first(Runnable printFirst) throws InterruptedException {

    // printFirst.run() outputs "first". Do not change or remove this line.
    printFirst.run();
    mutex2.release();
  }

  public void second(Runnable printSecond) throws InterruptedException {
    mutex2.acquire();
    // printSecond.run() outputs "second". Do not change or remove this line.
    printSecond.run();
    mutex2.release();
    mutex3.release();
  }

  public void third(Runnable printThird) throws InterruptedException {
    mutex3.acquire();
    // printThird.run() outputs "third". Do not change or remove this line.
    printThird.run();
    mutex3.release();
  }
}
