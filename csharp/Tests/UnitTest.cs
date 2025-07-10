using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("0", true)]
    [DataRow(".", false)]
    [DataRow("-1E+3", true)]
    [DataRow(".1", true)]
    [DataRow("+.8", true)]
    public void TestMethod1(string s, bool exp)
    {
        Assert.AreEqual(exp, sol.IsNumber(s));
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