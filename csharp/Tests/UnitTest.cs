using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("3", "13")]
    [DataRow("8", "4681")]
    [DataRow("999999999999999999", "1000000000000000000")]
    [DataRow("686286299", "470988884881403701")]
    public void TestMethod1(string exp, string n)
    {
        Assert.AreEqual(exp, sol.SmallestGoodBase(n));
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