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
        SnapshotArray arr = new(1);
        arr.Snap();
        arr.Set(0, 16);
        Assert.AreEqual(16, arr.Get(0, 2));
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