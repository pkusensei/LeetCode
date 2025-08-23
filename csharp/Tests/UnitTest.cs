using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    // readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        LFUCache lfu = new(2);
        lfu.Put(1, 1);   // cache=[1,_], cnt(1)=1
        lfu.Put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
        Assert.AreEqual(1, lfu.Get(1));      // return 1
                                             // cache=[1,2], cnt(2)=1, cnt(1)=2
        lfu.Put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                         // cache=[3,1], cnt(3)=1, cnt(1)=2
        Assert.AreEqual(-1, lfu.Get(2));      // return -1 (not found)
        Assert.AreEqual(3, lfu.Get(3));      // return 3
                                             // cache=[3,1], cnt(3)=2, cnt(1)=2
        lfu.Put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                         // cache=[4,3], cnt(4)=1, cnt(3)=2
        Assert.AreEqual(-1, lfu.Get(1));      // return -1 (not found)
        Assert.AreEqual(3, lfu.Get(3));      // return 3
                                             // cache=[3,4], cnt(4)=1, cnt(3)=3
        Assert.AreEqual(4, lfu.Get(4));      // return 4
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}