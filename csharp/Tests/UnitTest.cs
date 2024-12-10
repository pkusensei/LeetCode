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
        Skiplist skiplist = new Skiplist();
        skiplist.Add(1);
        skiplist.Add(2);
        skiplist.Add(3);
        Assert.IsFalse(skiplist.Search(0)); // return False
        skiplist.Add(4);
        Assert.IsTrue(skiplist.Search(1)); // return True
        Assert.IsFalse(skiplist.Erase(0));  // return False, 0 is not in skiplist.
        Assert.IsTrue(skiplist.Erase(1));  // return True
        Assert.IsFalse(skiplist.Search(1)); // return False, 1 has already been erased.
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