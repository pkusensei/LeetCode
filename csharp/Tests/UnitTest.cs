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
        RandomizedCollection c = new();
        Assert.IsTrue(c.Insert(10));
        Assert.IsFalse(c.Insert(10));
        Assert.IsTrue(c.Insert(20));
        Assert.IsFalse(c.Insert(20));
        Assert.IsTrue(c.Insert(30));
        Assert.IsFalse(c.Insert(30));
        Assert.IsTrue(c.Remove(10));
        Assert.IsTrue(c.Remove(20));
        Assert.IsTrue(c.Remove(20));
        Assert.IsTrue(c.Remove(10));
        Assert.IsTrue(c.Remove(30));
        Assert.IsTrue(c.Insert(40));
        Assert.IsTrue(c.Remove(30));
        Assert.IsFalse(c.Remove(30));
        Assert.AreEqual(40, c.GetRandom());
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