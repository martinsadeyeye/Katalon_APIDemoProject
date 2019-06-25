<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Generating RRR</description>
   <name>GenerateRRR</name>
   <tag></tag>
   <elementGuidId>16279699-a7fc-4dbb-951c-5eef8a5c65c4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;customFields\&quot;: [{\n\t\t\&quot;id\&quot;: \&quot;1509377912186\&quot;,\n\t\t\&quot;values\&quot;: [{\n\t\t\t\&quot;value\&quot;: \&quot;0101175047386\&quot;,\n\t\t\t\&quot;quntity\&quot;: 0,\n\t\t\t\&quot;amount\&quot;: 0\n\t\t}]\n\t}\n\t],\n\t\n\t\&quot;billId\&quot;: \&quot;1509377912187\&quot;,\n\t\&quot;billerId\&quot;: \&quot;C0000115401\&quot;,\n\t\&quot;amount\&quot;: 10000,\n\t\&quot;payerPhone\&quot;: \&quot;07038496242\&quot;,\n\t\&quot;currency\&quot;: \&quot;NGN\&quot;,\n\t\&quot;payerName\&quot;: \&quot;DANIS IGWE \&quot;,\n\t\&quot;payerEmail\&quot;: \&quot;awoedey2k@gmail.com\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>publicKey</name>
      <type>Main</type>
      <value>U0hFTEx8MTUwOTM3MTg1NDczOXwyNjdjNDBiZmI5ZjMzMjg5M2I3MWI2YzEzZWUxYTQ5YjUxOTRhMjY5ZDljOWUzNmI0MWUxOTgyYzI1NDUyYTMxM2NlM2QxYTdmZjQxMTExN2M5MTU1NjgxNWYyYmEwMTI3ZWY3MmU4M2MxNmE2ZjBmNjE3Y2Q2OTNlYzA1ODA4Nw=='</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>transactionId</name>
      <type>Main</type>
      <value>0001</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://192.9.200.209:6200/microservice/remita/gateway/send/api/bgatesvc/billing/generate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'responseData[0].amountDue', '10157.5')



WS.verifyElementPropertyValue(response, 'responseMsg', 'Successful')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
