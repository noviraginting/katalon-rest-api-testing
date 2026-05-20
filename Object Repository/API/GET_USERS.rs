<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_USERS</name>
   <tag></tag>
   <elementGuidId>90c6cb59-f401-407c-a7e8-b785b0b25235</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>free_user_3Dyngo1j44gMK4KgRuEmCNQtNfN</value>
      <webElementGuid>e7f2be4a-1ab9-4491-bfa8-0ab018bb67c1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*&#xd;
&#xd;
import com.kms.katalon.core.testobject.RequestObject&#xd;
import com.kms.katalon.core.testobject.ResponseObject&#xd;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS&#xd;
import com.kms.katalon.core.webservice.verification.WSResponseManager&#xd;
&#xd;
import groovy.json.JsonSlurper&#xd;
import internal.GlobalVariable as GlobalVariable&#xd;
&#xd;
RequestObject request = WSResponseManager.getInstance().getCurrentRequest()&#xd;
&#xd;
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()&#xd;
&#xd;
&#xd;
// VALIDASI STATUS CODE&#xd;
WS.verifyResponseStatusCode(response, 200)&#xd;
&#xd;
&#xd;
// PARSE JSON RESPONSE&#xd;
def jsonSlurper = new JsonSlurper()&#xd;
def responseBody = jsonSlurper.parseText(response.getResponseBodyContent())&#xd;
&#xd;
&#xd;
// VALIDASI DATA TIDAK KOSONG&#xd;
assert responseBody.data.size() > 0&#xd;
&#xd;
&#xd;
// PRINT RESPONSE&#xd;
println(response.getResponseBodyContent())</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
