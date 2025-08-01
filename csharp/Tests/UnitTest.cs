using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, 1, 2)]
    [DataRow(7, 3, 4)]
    public void TestMethod1(int exp, int a, int b)
    {
        Assert.AreEqual(exp, sol.GetSum(a, b));
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